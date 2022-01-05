use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};

use pyo3::prelude::{Py, PyAny, PyModule, Python};
use pyo3::FromPyObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RequestData {
    inputs: String,
}
#[derive(FromPyObject, Debug, Serialize)]
struct TextClassificationResponse {
    #[pyo3(item)]
    label: String,
    #[pyo3(item)]
    score: f32,
}

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index(text_classification: Data<Py<PyAny>>, r: web::Json<RequestData>) -> HttpResponse {
    println!("{}", r.inputs);
    let pred = Python::with_gil(|py| {
        text_classification
            .call1(py, ("i love you.",))
            .unwrap()
            .extract::<Vec<TextClassificationResponse>>(py)
            .unwrap()
    });
    println!("{:?}", pred);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(pred)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let text_classification: Py<PyAny> = Python::with_gil(|py| {
        let transformers = PyModule::import(py, "transformers").unwrap();
        return transformers
            .getattr("pipeline")
            .unwrap()
            .call1(("text-classification",))
            .unwrap()
            .extract()
            .unwrap();
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(text_classification.clone())) // add thread-local state
            .service(
                web::resource("/age")
                    // change json extractor configuration
                    .route(web::post().to(index)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
