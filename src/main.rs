extern crate clap;
#[macro_use]
extern crate rouille;
extern crate serde;

use std::sync::Arc;

use clap::{App, Arg, crate_authors, crate_name, crate_version, Values};

use crate::api::router;
use crate::pokemon::repository::repository::{AirtableRepository, InMemoryRepository, Repository, SqliteRepository};

mod pokemon;
mod api;
mod cli;

fn main() {
    // Without CLI
    // let repo = Arc::new(InMemoryRepository::new());
    // router::serve("localhost:8000", repo);

    // With CLI
    // RUN:
    //     cargo run -- --help
    // USAGE:
    //     rust-hexagonal-architecture [FLAGS]
    //
    // FLAGS:
    //         --cli        Runs in CLI mode
    //     -h, --help       Prints help information
    //     -V, --version    Prints version information
    //
    // let repo = Arc::new(InMemoryRepository::new());

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("cli").long("cli").help("Runs in CLI mode"))
        .arg(Arg::with_name("sqlite").long("sqlite").value_name("PATH"))
        .arg(Arg::with_name("airtable").long("airtable").value_names(&["API_KEY", "WORKSPACE_ID"]))
        .get_matches();

    // With CLI multi repo
    let repo = build_repo(matches.value_of("sqlite"), matches.values_of("airtable"));

    match matches.occurrences_of("cli") {
        0 => router::serve("localhost:8000", repo),
        _ => cli::run(repo),
    }
}

fn build_repo(sqlite_value: Option<&str>, airtable_values: Option<Values>) -> Arc<dyn Repository> {
    if let Some(values) = airtable_values {
        if let [api_key, workspace_id] = values.collect::<Vec<&str>>()[..] {
            match AirtableRepository::try_new(api_key, workspace_id) {
                Ok(repo) => return Arc::new(repo),
                _ => panic!("Error while creating airtable repo"),
            }
        }
    }

    if let Some(path) = sqlite_value {
        match SqliteRepository::try_new(path) {
            Ok(repo) => return Arc::new(repo),
            _ => panic!("Error while creating sqlite repo"),
        }
    }

    Arc::new(InMemoryRepository::new())
}