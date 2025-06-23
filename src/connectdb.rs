use std::error::Error;
use std::sync::LazyLock;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub struct Database {
    connected: bool,
    pub db: &'static Surreal<Client>,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Connect to WebSocket
        DB.connect::<Ws>("127.0.0.1:8000").await?;

        DB.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        DB.use_ns("eric").use_db("Xone").await?;
        println!("connected to DB");

        Ok(Self { 
            connected: true,
            db: &DB
        })
    }
}