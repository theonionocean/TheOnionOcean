use std::future::Future;

use pass_me_core::ValidationError;

use crate::{mediatr_error::MediatrError, Command, Query};

pub trait QueryHandler<Q: Query>: Send + Sync {
    fn validate(
        &self,
        _query: &Q,
    ) -> impl Future<Output = Vec<ValidationError>> + Send {
        async { Vec::new() }
    }

    fn handle(
        &self,
        query: Q,
    ) -> impl Future<Output = Result<Q::Response, MediatrError>> + Send;
}

pub trait CommandHandler<C: Command>: Send + Sync {
    fn validate(
        &self,
        _command: &C,
    ) -> impl Future<Output = Vec<ValidationError>> + Send {
        async { Vec::new() }
    }

    fn handle(
        &self,
        command: C,
    ) -> impl Future<Output = Result<C::Response, MediatrError>> + Send;
}
