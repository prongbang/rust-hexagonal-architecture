use std::sync::Arc;

use rouille;
use serde::Serialize;

use crate::api::health::Status;
use crate::pokemon::domain::fetch_all_pokemons;
use crate::pokemon::repository::repository::Repository;

#[derive(Serialize)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn serve(repo: Arc<dyn Repository>) -> rouille::Response {
    match fetch_all_pokemons::execute(repo) {
        Ok(res) => rouille::Response::json(
            &res.into_iter()
                .map(|p| Response {
                    number: p.number,
                    name: p.name,
                    types: p.types,
                })
                .collect::<Vec<Response>>(),
        ),
        Err(fetch_all_pokemons::Error::Unknown) => {
            rouille::Response::from(Status::InternalServerError)
        }
    }
}