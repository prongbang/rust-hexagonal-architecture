use std::sync::Arc;

use crate::api::{create_pokemon, delete_pokemon, fetch_all_pokemons, fetch_pokemon, health};
use crate::api::health::Status;
use crate::pokemon::repository::repository::Repository;

pub fn serve(url: &str, repo: Arc<dyn Repository>) {
    rouille::start_server(url, move |req| {
        router!(req,
            (GET) (/) => {
                fetch_all_pokemons::serve(repo.clone())
            },
            (POST) (/) => {
                create_pokemon::serve(repo.clone(), req)
            },
            (GET) (/health) => {
                health::serve()
            },
            (GET) (/{number: u16}) => {
                fetch_pokemon::serve(repo.clone(), number)
            },
            (DELETE) (/{number: u16}) => {
                delete_pokemon::serve(repo.clone(), number)
            },
            _ => {
                rouille::Response::from(Status::NotFound)
            }
        )
    });
}
