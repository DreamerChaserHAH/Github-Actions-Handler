///<summary>
/// This script will contain the schema for the hosting
///</summary>

use actix_web::{get, web, App, HttpResponse, HttpServer, HttpRequest};

use serde_derive::Deserialize;

use crate::api_schema;

#[derive(Debug, Deserialize)]
pub struct Params {
    command: String
}

#[get("/")]
async fn command_runner(req: HttpRequest) -> HttpResponse {
    let params = web::Query::<Params>::from_query(req.query_string());
    if params.is_ok(){
        let command_id = params.unwrap().command.to_owned();
        
        let process_command_result = api_schema::process_command(&command_id).await;
        
        if process_command_result.is_ok() {
            HttpResponse::Ok().body("Command has been executed")
        }else{
            eprintln!("{:?}", process_command_result.err());
            HttpResponse::Ok().body("Some Error has occured! Check Server Log for details")
        }
    }else{
        HttpResponse::Forbidden().body("No value!")
    }
}

pub async fn start_server(_port: i32) -> std::io::Result<()>{

    println!("Starting Server on: {}", _port);

    HttpServer::new(|| {
        App::new()
            .service(command_runner)
    })
    .bind(("127.0.0.1", _port as u16))?
    .run()
    .await
}