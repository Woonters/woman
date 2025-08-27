use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fmt::Error;
use surrealdb::Error as surrealError;
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};

type DbStr = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    name: DbStr,
    tldr: DbStr,
    info: DbStr,
    common_uses: DbStr,
    resources: DbStr,
    extra: DbStr,
}

impl Entry {
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "# TLDR \n{}\n\n# Info \n{}\n\n# Common Uses \n{}\n\n# Resources \n{}\n\n# {}",
            self.tldr.trim_end(),
            self.info.trim_end(),
            self.common_uses.trim_end(),
            self.resources.trim_end(),
            self.extra.trim_end()
        )
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            name: "name".to_owned(),
            tldr: "\n What the application does in one to two sentences".to_owned(),
            info: "More information, generally useful tips and tricks\n- could\n- use\n- a\n- list".to_owned(),
            common_uses: "\n## All of these can use subheadings\n\n`woman-e <app>`\nexamples with text explaining what they do".to_owned(),
            resources: "\nAny links?\nhttps://github.com/woonters/woman".to_owned(),
            extra: "Extra\nAnything can go here, you can even edit the word 'Extra'".to_owned(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Record {
    id: RecordId,
}

/// confirm there is a db file somewhere
///
/// it should check:
/// .cache/woman/db
/// ...
///
/// # Errors
///
/// File system errors
pub async fn check_cache() -> Result<(), Error> {
    todo!()
}

/// Open the database, if none exist then it should create one at .cache/woman/db
///
/// # Errors
///
/// This function will return an error if the database fails to open or be created
pub async fn setup_db() -> Result<Surreal<Db>, surrealError> {
    // TODO if there is no database then create one
    let db = Surreal::new::<RocksDb>("~/.cache/woman/db").await?;
    db.use_ns("app").use_db("data").await?;
    Ok(db)
}
