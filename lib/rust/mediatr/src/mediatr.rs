use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
};

use crate::{mediatr_error::MediatrError, Command, CommandHandler, Query, QueryHandler};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

struct QueryHandlerWrapper<H, M> {
    handler: H,
    _message: PhantomData<M>,
}

impl<H, M> DynamicHandler for QueryHandlerWrapper<H, M>
where
    H: QueryHandler<M> + 'static,
    M: Query + Send + Sync + 'static,
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
            let errors = self.handler.validate(&request).await;
            if !errors.is_empty() {
                return Err(MediatrError::ValidationFailed(errors));
            }
            let response = self.handler.handle(request).await?;
            Ok(Box::new(response) as Box<dyn Any + Send>)
        })
    }
}

struct CommandHandlerWrapper<H, M> {
    handler: H,
    _message: PhantomData<M>,
}

impl<H, M> DynamicHandler for CommandHandlerWrapper<H, M>
where
    H: CommandHandler<M> + 'static,
    M: Command + Send + Sync + 'static,
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
            let errors = self.handler.validate(&request).await;
            if !errors.is_empty() {
                return Err(MediatrError::ValidationFailed(errors));
            }
            let response = self.handler.handle(request).await?;
            Ok(Box::new(response) as Box<dyn Any + Send>)
        })
    }
}

trait DynamicHandler: Send + Sync {
    fn handle(
        &self,
        request: Box<dyn Any + Send>,
    ) -> BoxFuture<'_, Result<Box<dyn Any + Send>, MediatrError>>;
}

#[derive(Default, Clone)]
pub struct Mediatr {
    handlers: HashMap<TypeId, Arc<dyn DynamicHandler>>,
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
            Arc::new(QueryHandlerWrapper {
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
            Arc::new(CommandHandlerWrapper {
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
