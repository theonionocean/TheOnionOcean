use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    marker::PhantomData,
    pin::Pin,
};

use crate::{mediatr_error::MediatrError, Command, CommandHandler, Query, QueryHandler};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

macro_rules! define_handler_wrapper {
    ($wrapper:ident, $handler_trait:ident, $message_trait:ident) => {
        struct $wrapper<H, M> {
            handler: H,
            _message: PhantomData<M>,
        }

        impl<H, M> DynamicHandler for $wrapper<H, M>
        where
            H: $handler_trait<M> + 'static,
            M: $message_trait + Send + Sync + 'static,
            M::Response: Send + 'static,
        {
            fn handle(
                &self,
                request: Box<dyn Any + Send>,
            ) -> BoxFuture<'_, Result<Box<dyn Any + Send>, MediatrError>> {
                Box::pin(async move {
                    let request = *request.downcast::<M>().map_err(|_| {
                        MediatrError::HandlerNotFound(std::any::type_name::<M>().to_string())
                    })?;
                    let response = self.handler.handle(request).await?;
                    Ok(Box::new(response) as Box<dyn Any + Send>)
                })
            }
        }
    };
}

define_handler_wrapper!(QueryHandlerWrapper, QueryHandler, Query);
define_handler_wrapper!(CommandHandlerWrapper, CommandHandler, Command);

trait DynamicHandler: Send + Sync {
    fn handle(
        &self,
        request: Box<dyn Any + Send>,
    ) -> BoxFuture<'_, Result<Box<dyn Any + Send>, MediatrError>>;
}

#[derive(Default)]
pub struct Mediatr {
    handlers: HashMap<TypeId, Box<dyn DynamicHandler>>,
}

impl Mediatr {
    pub fn register_query<H, Q>(&mut self, handler: H)
    where
        H: QueryHandler<Q> + 'static,
        Q: Query + Send + Sync + 'static,
        Q::Response: Send + 'static,
    {
        self.handlers.insert(
            TypeId::of::<Q>(),
            Box::new(QueryHandlerWrapper {
                handler,
                _message: PhantomData,
            }),
        );
    }

    pub fn register_command<H, C>(&mut self, handler: H)
    where
        H: CommandHandler<C> + 'static,
        C: Command + Send + Sync + 'static,
        C::Response: Send + 'static,
    {
        self.handlers.insert(
            TypeId::of::<C>(),
            Box::new(CommandHandlerWrapper {
                handler,
                _message: PhantomData,
            }),
        );
    }

    pub async fn send_query<Q>(&self, query: Q) -> Result<Q::Response, MediatrError>
    where
        Q: Query + Send + Sync + 'static,
        Q::Response: Send + 'static,
    {
        self.dispatch::<Q, Q::Response>(query).await
    }

    pub async fn send_command<C>(&self, command: C) -> Result<C::Response, MediatrError>
    where
        C: Command + Send + Sync + 'static,
        C::Response: Send + 'static,
    {
        self.dispatch::<C, C::Response>(command).await
    }

    async fn dispatch<M, R>(&self, message: M) -> Result<R, MediatrError>
    where
        M: Send + 'static,
        R: Send + 'static,
    {
        let message_type = TypeId::of::<M>();

        let handler = self
            .handlers
            .get(&message_type)
            .ok_or(MediatrError::HandlerNotFound(
                std::any::type_name::<M>().to_string(),
            ))?;

        let response = handler
            .handle(Box::new(message))
            .await?
            .downcast::<R>()
            .map_err(|_| MediatrError::HandlerNotFound(std::any::type_name::<M>().to_string()))?;

        Ok(*response)
    }
}
