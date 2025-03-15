mod api;
mod colors;
mod estimator;
mod types;

use crate::api::{estimate::estimate, index::index};
use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index).service(estimate);
    };

    Ok(config.into())
}
