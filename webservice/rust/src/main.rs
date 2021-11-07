use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct Human {
    #[serde(default)]
    name: String,
    #[serde(default)]
    age: u8,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct ResponseBody {
    message: String,
    born: u16,
}

#[post("/age")]
async fn age(body: web::Json<Human>) -> Result<HttpResponse, Error> {
    // calculate born year

    let born: u16 = 2021 - (body.age as u16);

    // create Response struct
    let res = ResponseBody {
        message: format!("{name} was born in {born}", name = body.name, born = born),
        born,
    };

    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(age))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
