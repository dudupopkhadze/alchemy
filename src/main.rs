
mod  compiler;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use rocket::http::Status;
use serde::Serialize;
use rocket::serde::json::Json;


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
    rocket::build().mount("/", routes![index, compile])
}