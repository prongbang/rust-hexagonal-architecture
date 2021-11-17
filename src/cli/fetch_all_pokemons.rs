use std::sync::Arc;

use crate::pokemon::domain::fetch_all_pokemons;
use crate::pokemon::repository::repository::Repository;

#[derive(Debug)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn run(repo: Arc<dyn Repository>) {
    match fetch_all_pokemons::execute(repo) {
        Ok(res) => res.into_iter().for_each(|p| {
            println!(
                "{:?}",
                Response {
                    number: p.number,
                    name: p.name,
                    types: p.types,
                }
            );
        }),
        Err(fetch_all_pokemons::Error::Unknown) => println!("An unknown error occurred"),
    };
}