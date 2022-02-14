use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};

use pyo3::prelude::{Py, PyAny, PyModule, Python};
use pyo3::types::IntoPyDict;
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
async fn index(handler: Data<Py<PyAny>>, r: web::Json<RequestData>) -> HttpResponse {
    // println!("{}", r.inputs);
    // let now = std::time::Instant::now();
    let pred = Python::with_gil(|py| {
        // println!("{}", now.elapsed().as_millis());
        // let now = std::time::Instant::now();
        let res = handler
            .call1(py, (r.inputs.as_str(),))
            .unwrap()
            .extract::<TextClassificationResponse>(py)
            .unwrap();
        // println!("prediction took: {}ms", now.elapsed().as_millis());
        res
    });
    // println!("{:?}", pred);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(pred)
}

type PyFunction = Py<PyAny>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PYTHON_MODULE: &str = include_str!("../handler.py");

    let pipeline: PyFunction = Python::with_gil(|py| {
        let pipeline = PyModule::from_code(py, PYTHON_MODULE, "handler.py", "handler").unwrap();
        let handler = pipeline.getattr("handle").unwrap();
        handler.extract().unwrap()
    });
    println!("model loaded");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pipeline.clone())) // add thread-local state
            .service(
                web::resource("/age")
                    // change json extractor configuration
                    .route(web::post().to(index)),
            )
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
