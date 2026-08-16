use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Clone)]
pub struct DatabaseContext {
    db: Surreal<Client>,
}

impl DatabaseContext {
    pub async fn new(
        host: String,
        username: String,
        password: String,
        ns: String,
        db: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let database = Surreal::new::<Ws>(host).await?;

        database
            .signin(Root {
                username: username,
                password: password,
            })
            .await?;

        database.use_ns(ns).use_db(db).await?;

        Ok(Self { db: database })
    }

    pub fn db(&self) -> &Surreal<Client> {
        &self.db
    }
}
