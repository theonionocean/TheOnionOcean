use crate::{mediatr_error::MediatrError, Command, Query};

pub trait QueryHandler<Q: Query> {
    fn handle(query: Q) -> Result<Q::Response, MediatrError>;
}

pub trait CommandHandler<C: Command> {
    fn handle(command: C) -> Result<C::Response, MediatrError>;
}
