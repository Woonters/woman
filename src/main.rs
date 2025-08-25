use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::env::var;
use std::fmt::Error;
use std::fs::File;
use std::fs::remove_file;
use std::io::Read;
use std::io::Write;
use std::process::Command;

use atty::Stream;
use clap::Parser;
use surrealdb::Error as surrealError;
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};

use serde::{Deserialize, Serialize};
use tokio::io::Repeat;

mod parser;

// type DbStr = Cow<'static, str>;
type DbStr = String;

static EXAMPLE: &str = r"
# TLDR

What the application does in one to too sentences

# Info

More information, generally useful tips and tricks
- could
- use
- a
- list

# Common Uses

`woman -e woman`
examples and a quick explaination
i.e. edits the woman entry 

# Resources

Any links?
https://github.com/woonters/woman

# Extra

Anything more to add here, this can have anything following it so really just write anything (even edit 'Extra')
";
#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    name: DbStr,
    tldr: DbStr,
    info: DbStr,
    common_uses: DbStr,
    resources: DbStr,
    extra: DbStr,
}

impl Entry {
    fn to_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    fn to_string(self) -> String {
        let mut formatted_string = String::new();
        formatted_string.push_str("# TLDR \n");
        formatted_string.push_str(&clean_string_end(&self.tldr));
        formatted_string.push_str("# Info \n");
        formatted_string.push_str(&clean_string_end(&self.info));
        formatted_string.push_str("# Common Uses\n");
        formatted_string.push_str(&clean_string_end(&self.common_uses));
        formatted_string.push_str("# Resources \n");
        formatted_string.push_str(&clean_string_end(&self.resources));
        formatted_string.push_str("# ");
        formatted_string.push_str(&clean_string_end(&self.extra));
        formatted_string
    }
}

/// Cleans up text sections to just have a single trailing newline, currently really
/// ineffcient
fn clean_string_end(s: &String) -> String {
    let mut cleaned = s.trim_end().to_string();
    cleaned.push_str("\n\r");
    cleaned
}

#[derive(Debug, Deserialize)]
struct Record {
    id: RecordId,
}

#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
struct Args {
    /// The application name
    name: String,
    /// Edit the woman page for the selected application
    #[arg(short, long)]
    edit: bool,

    /// Print the woman page for the selected application to the tty instead of opening the reader
    #[arg(short, long, default_value_t = false)]
    print: bool,
}

#[tokio::main]
async fn main() -> Result<(), surrealError> {
    let args = Args::parse();
    // TODO: sanitise args.name

    let db = setup_db().await?;

    // find out if there is an existing page for the application
    let resp: Option<Entry> = db.select(("entry", &args.name)).await?;

    if args.edit {
        let mut f = File::create_new("WOMANEDIT").expect("Tried to create a file when WOMANEDIT already existed, please delete any file called WOMANEDIT in the current space as this is used for the editing buffer");
        match resp {
            Some(resp) => {
                // TODO Entry to bytes function
                f.write_all(&resp.to_bytes())
                    .expect("Write error on creating default WOMANEDIT");
            }
            None => {
                f.write_all(EXAMPLE.as_bytes())
                    .expect("Write error on creating default WOMANEDIT");
            }
        }
        let editor = var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        // TODO: If this fails we should still clean up WOMANEDIT file
        Command::new(editor)
            .arg("WOMANEDIT")
            .status()
            .expect("Opening the editor failed ");
        let mut edited = String::new();
        File::open("WOMANEDIT")
            .expect("Couldn't open WOMANEDIT file did you delete it?")
            .read_to_string(&mut edited)
            .expect("Edited file couldn't be read as string, have you added invalid characters");
        let entry = parser::parse_entry(&mut &edited[..], &args.name);
        let _: Option<Record> = db.insert(("entry", &args.name)).content(entry).await?;
        remove_file("WOMANEDIT")
            .expect("Couldn't delete WOMANEDIT file, you might need to do this");
    } else {
        match resp {
            Some(resp) => {
                println!("{}", resp.to_string());
            }
            None => {
                println!("No entry for {}", &args.name);
            }
        }
        // if args.print || atty::isnt(Stream::Stdout) {
        // } else {
        // }
    }

    Ok(())
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
async fn check_cache() -> Result<(), Error> {
    todo!()
}

/// Open the database, if none exist then it should create one at .cache/woman/db
///
/// # Errors
///
/// This function will return an error if the database fails to open or be created
async fn setup_db() -> Result<Surreal<Db>, surrealError> {
    // TODO if there is no database then create one
    let db = Surreal::new::<RocksDb>("~/.cache/woman/db").await?;
    db.use_ns("app").use_db("data").await?;
    Ok(db)
}
