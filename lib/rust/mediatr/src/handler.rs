use std::future::Future;

use crate::{mediatr_error::MediatrError, Command, Query};

pub trait QueryHandler<Q: Query>: Send + Sync {
    fn handle(
        &self,
        query: Q,
    ) -> impl Future<Output = Result<Q::Response, MediatrError>> + Send;
}

pub trait CommandHandler<C: Command>: Send + Sync {
    fn handle(
        &self,
        command: C,
    ) -> impl Future<Output = Result<C::Response, MediatrError>> + Send;
}
