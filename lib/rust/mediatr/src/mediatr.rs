use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use crate::{mediatr_error::MediatrError, Command, CommandHandler, Query, QueryHandler};

macro_rules! define_handler_wrapper {
    ($wrapper:ident, $handler_trait:ident, $message_trait:ident) => {
        struct $wrapper<H, M> {
            _handler: PhantomData<H>,
            _message: PhantomData<M>,
        }

        impl<H, M> DynamicHandler for $wrapper<H, M>
        where
            H: $handler_trait<M> + 'static,
            M: $message_trait + 'static,
            M::Response: 'static,
        {
            fn handle(&self, request: Box<dyn Any>) -> Result<Box<dyn Any>, MediatrError> {
                let request = *request.downcast::<M>().map_err(|_| {
                    MediatrError::HandlerNotFound(std::any::type_name::<M>().to_string())
                })?;
                let response = H::handle(request)?;
                Ok(Box::new(response))
            }
        }
    };
}

define_handler_wrapper!(QueryHandlerWrapper, QueryHandler, Query);
define_handler_wrapper!(CommandHandlerWrapper, CommandHandler, Command);

trait DynamicHandler {
    fn handle(&self, request: Box<dyn Any>) -> Result<Box<dyn Any>, MediatrError>;
}

#[derive(Default)]
pub struct Mediatr {
    handlers: HashMap<TypeId, Box<dyn DynamicHandler>>,
}

impl Mediatr {
    pub fn register_query<H, Q>(&mut self, _handler: H)
    where
        H: QueryHandler<Q> + 'static,
        Q: Query + 'static,
        Q::Response: 'static,
    {
        self.handlers.insert(
            TypeId::of::<Q>(),
            Box::new(QueryHandlerWrapper {
                _handler: PhantomData::<H>,
                _message: PhantomData,
            }),
        );
    }

    pub fn register_command<H, C>(&mut self, _handler: H)
    where
        H: CommandHandler<C> + 'static,
        C: Command + 'static,
        C::Response: 'static,
    {
        self.handlers.insert(
            TypeId::of::<C>(),
            Box::new(CommandHandlerWrapper {
                _handler: PhantomData::<H>,
                _message: PhantomData,
            }),
        );
    }

    pub fn send_query<Q>(&self, query: Q) -> Result<Q::Response, MediatrError>
    where
        Q: Query + 'static,
        Q::Response: 'static,
    {
        self.dispatch::<Q, Q::Response>(query)
    }

    pub fn send_command<C>(&self, command: C) -> Result<C::Response, MediatrError>
    where
        C: Command + 'static,
        C::Response: 'static,
    {
        self.dispatch::<C, C::Response>(command)
    }

    fn dispatch<M, R>(&self, message: M) -> Result<R, MediatrError>
    where
        M: 'static,
        R: 'static,
    {
        let message_type = TypeId::of::<M>();

        let handler = self
            .handlers
            .get(&message_type)
            .ok_or(MediatrError::HandlerNotFound(
                std::any::type_name::<M>().to_string(),
            ))?;

        let response = handler
            .handle(Box::new(message))?
            .downcast::<R>()
            .map_err(|_| MediatrError::HandlerNotFound(std::any::type_name::<M>().to_string()))?;

        Ok(*response)
    }
}
