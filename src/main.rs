use std::env::var;
use std::fs::File;
use std::fs::remove_file;
use std::io::Read;
use std::io::Write;
use std::process::Command;

use atty::Stream;
use clap::Parser;
use surrealdb::Error as surrealError;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

use crate::data_base::Entry;
use crate::data_base::Record;
use crate::data_base::setup_db;
use crate::display::display_setup;

mod data_base;
mod display;
mod parser;

// TODO: Better documentation of commandline arguments
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

    /// Print just the common uses for the selected application, if editing it will only edit that section of the document
    #[arg(short, long, default_value_t = false)]
    common_uses: bool,

    /// Print just the tldr for the selected application, if editing it will only edit that section of the document
    #[arg(short, long, default_value_t = false)]
    tldr: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<surrealError>> {
    let args = Args::parse();
    // TODO: sanitise args.name

    let db = setup_db().await?;

    // find out if there is an existing page for the application
    let resp: Option<Entry> = db.select(("entry", &args.name)).await?;

    if args.edit {
        edit_entry(resp, &args.name, db).await;
    } else {
        match resp {
            Some(resp) => {
                if args.print || atty::isnt(Stream::Stdout) {
                    println!("{}", resp);
                } else {
                    display_setup(&resp).unwrap();
                }
            }
            None => {
                println!("No entry for {}", &args.name);
            }
        }
    }

    Ok(())
}

enum EditError {
    FileCreationFailure,
    FileUseageFailure(String),
    ParsingError(String),
    DBError,
}

async fn edit_entry(entry: Option<Entry>, appname: &str, db: Surreal<Any>) {
    match _edit_entry(entry, appname, db).await {
        Ok(_) => {
            remove_file("WOMANEDIT").expect("Failed to cleanup the WOMANEDIT file, have you deleted it yourself? or have permissions changed?");
        }
        Err(EditError::DBError) => {
            panic!("Database Error whilst saving the entered data")
        }

        Err(EditError::FileUseageFailure(s)) | Err(EditError::ParsingError(s)) => {
            // is this the best way to do this?
            let l = remove_file("WOMANEDIT").map(|_| "".to_owned()).unwrap_or_else(|_| "and Failed to cleanup the WOMANEDIT file, have you deleted it yourself? or have permissions changed?".to_owned());
            panic!("{} {} ", s, l)
        }

        Err(EditError::FileCreationFailure) => {
            panic!(
                "Failed to create WOMANEDIT file, does woman have permissions in this folder? is there a WOMANEDIT file already?"
            )
        }
    }
}

async fn _edit_entry(
    entry: Option<Entry>,
    appname: &str,
    db: Surreal<Any>,
) -> Result<(), EditError> {
    let mut f = File::create_new("WOMANEDIT").map_err(|_| EditError::FileCreationFailure)?;
    // it has probably errored out due to file permissions or available size?
    f.write_all(&entry.unwrap_or_default().into_bytes())
        .map_err(|_| EditError::FileUseageFailure("Failed to write to WOMANEDIT".to_owned()))?;
    // TODO: maybe use f.sync_data() or f.sync_all() here to avoid opening the editor on the file before it is ready
    let editor = var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    Command::new(editor)
        .arg("WOMANEDIT")
        .status()
        .map_err(|_| EditError::FileUseageFailure("Opening the editor failed".to_owned()))?;
    let mut edited = String::new();
    File::open("WOMANEDIT")
        .expect("Couldn't open WOMANEDIT file did you delete it?")
        .read_to_string(&mut edited)
        .map_err(|_| {
            EditError::FileUseageFailure(
                "Edited file couldn't be read as string, have you added invalid characters?"
                    .to_owned(),
            )
        })?;
    let entry = parser::parse_entry(&mut &edited[..], appname);
    let _: Option<Record> = db
        .insert(("entry", appname))
        .content(entry)
        .await
        .map_err(|_| EditError::DBError)?;
    Ok(())
}
