
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use actix_web::{
    web::{
        scope,
        Json,
        Path,
        Data,
        ServiceConfig
    },
    get,
    post,
    HttpResponse,
    Responder,
};

use serde_json::json;



#[get("/json_parser")]
async fn json_parser() -> impl Responder {

        let mut file = File::open("mock.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data);
        let object: Value = serde_json::from_str(&data).unwrap();
      
        println!("{:?}", object);

        HttpResponse::Ok().json(json!({
            "status": "success",
            "message": object
        }))
}


pub fn config(conf:  &mut ServiceConfig) {
    let scope = scope("/api")
                .service(json_parser);



    conf.service(scope);
}