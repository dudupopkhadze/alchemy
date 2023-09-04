
mod  compiler;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use rocket::http::{Status, Method, Header};
use serde::Serialize;
use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "*",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[derive(Serialize, Deserialize)]
struct CodePayload {
    code: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[post("/", format = "json", data = "<code_payload>")]
fn compile(code_payload: Json<CodePayload>) -> Result<Json<compiler::CompileResult>, Status> {
    let compile_result = compiler::compile_and_run(&code_payload.code);
    if compile_result.error.is_empty() {
        Ok(Json(compile_result))
    } else {
        Err(Status::InternalServerError)
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![index, compile])
}