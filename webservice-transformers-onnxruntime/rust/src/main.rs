use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};

use onnxruntime::environment::Environment;
use onnxruntime::ndarray::prelude::*;
use onnxruntime::tensor;
use onnxruntime::{session::Session, LoggingLevel};
use std::cmp::Ordering;
use std::sync::Mutex;
use tokenizers::tokenizer::Tokenizer;

use serde::{Deserialize, Serialize};

// This struct represents state
struct AppState {
    session: Mutex<Session>,
    tokenizer: Tokenizer,
}

#[derive(Deserialize)]
struct RequestData {
    inputs: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct ResponseBody {
    label: String,
    score: f32,
}
/// deserialize `Info` from request's body, max payload size is 4kb
async fn index(state: web::Data<AppState>, r: web::Json<RequestData>) -> HttpResponse {
    let input = {
        let encoding = state.tokenizer.encode(r.inputs.as_str(), true).unwrap();

        let ids = encoding
            .get_ids()
            .iter()
            .map(|x| *x as i64)
            .collect::<Vec<i64>>();
        let id_array = Array::from_shape_vec((1, ids.len()), ids).unwrap();

        let attention_mask = encoding
            .get_attention_mask()
            .iter()
            .map(|x| *x as i64)
            .collect::<Vec<i64>>();

        let mask_array = Array::from_shape_vec((1, attention_mask.len()), attention_mask).unwrap();
        vec![id_array, mask_array]
    };

    let mut session = state.session.lock().unwrap();
    let _outputs: Vec<tensor::OrtOwnedTensor<f32, _>> = session.run(input).unwrap();

    let score_vec = _outputs[0]
        .softmax(onnxruntime::ndarray::Axis(1))
        .into_iter()
        .collect::<Vec<f32>>();

    let label_idx = score_vec
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(ResponseBody {
            label: format!("{}", label_idx),
            score: score_vec[label_idx],
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let environment = Environment::builder()
            .with_name("app")
            .with_log_level(LoggingLevel::Verbose)
            .build()
            .unwrap();

        let tokenizer = Tokenizer::from_file("tokenizer.json").unwrap();

        let state = AppState {
            tokenizer,
            session: Mutex::new(
                environment
                    .new_session_builder()
                    .unwrap()
                    .with_model_from_file("./pt_model.onnx")
                    .unwrap(),
            ),
        };
        App::new()
            .app_data(Data::new(state)) // add thread-local state
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
