use std::borrow::Cow;

use atty::Stream;
use std::io::{self, IsTerminal, Write};
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Entry {
    name: Cow<'static, str>,
    info: Cow<'static, str>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    if atty::is(Stream::Stdout) {
        // we are the current thing printing to tty
        todo!();
    } else {
        // we are piping to something else
        todo!();
    }
    let db = setup_db().await?;
    Ok(())
}

async fn setup_db() -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("namespace").use_db("database").await?;
    Ok(db)
}
