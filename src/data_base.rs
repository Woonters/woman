use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Text;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fmt::Error;
use std::fs::read_dir;
use std::io::ErrorKind;
use std::path::PathBuf;
use surrealdb::Error as surrealError;
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};

type DbStr = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub name: DbStr,
    pub tldr: DbStr,
    pub info: DbStr,
    pub common_uses: DbStr,
    pub resources: DbStr,
    pub extra: DbStr,
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
        // the trim_ends shouldn't be needed since they are now done at the parsing step
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
pub fn check_cache() -> Result<(), Error> {
    if read_dir("~/.cache/woman/db").is_ok() {
        return Ok(());
    }
    Err(Error)
}

/// Open the database, if none exist then it should create one at .cache/woman/db
///
/// # Errors
///
/// This function will return an error if the database fails to open or be created
pub async fn setup_db() -> Result<Surreal<Db>, surrealError> {
    // TODO if there is no database then create one
    // let _ = check_cache().map_err(|_| {
    //     surrealError::Api(surrealdb::error::Api::FileRead {
    //         path: PathBuf::from("~/.cache/woman/db"),
    //         error: std::io::Error::new(ErrorKind::NotFound, "Couldn't find db cache"),
    //     })
    // });
    let db = Surreal::new::<RocksDb>("~/.cache/woman/db").await?;
    db.use_ns("app").use_db("data").await?;
    Ok(db)
}
