use rouille;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

pub enum Status {
    Ok,
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}

impl From<Status> for rouille::Response {
    fn from(status: Status) -> Self {
        let status_code = match status {
            Status::Ok => 200,
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::Conflict => 409,
            Status::InternalServerError => 500,
        };
        Self {
            status_code,
            headers: vec![],
            data: rouille::ResponseBody::empty(),
            upgrade: None,
        }
    }
}

pub fn serve() -> rouille::Response {
    rouille::Response::json(&Response {
        message: String::from("Gotta catch them all!"),
    })
}