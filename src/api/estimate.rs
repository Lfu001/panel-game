use crate::colors::{Color, ColorMap};
use crate::estimator::to_entropy;
use crate::types::Rectangle;
use crate::{estimator::estimate_probabilities, types::Grid};
use actix_web::HttpResponse;
use actix_web::{Responder, post, web};
use serde::{Deserialize, Serialize};

const MAX_GRID_ROWS: usize = 9;
const MAX_GRID_COLS: usize = 9;

/// The request parameters.
#[derive(Serialize, Deserialize)]
struct RequestParams {
    /// A mask of the grid.
    mask: Grid<bool>,
    /// A list of rectangles to be placed.
    rectangles: Vec<Rectangle>,
}

/// The response message.
#[derive(Serialize, Deserialize)]
struct ResponseMessage {
    probabilities: Grid<(f64, Color)>,
    entropy: Grid<(f64, Color)>,
}

/// Checks if the grid size is valid.
fn validate_grid_size(grid: &Grid<bool>) -> bool {
    grid.rows() <= MAX_GRID_ROWS && grid.cols() <= MAX_GRID_COLS
}

#[post("/estimate")]
pub async fn estimate(param: web::Json<RequestParams>) -> impl Responder {
    if !validate_grid_size(&param.mask) {
        return HttpResponse::BadRequest().finish();
    }

    let probabilities = estimate_probabilities(&param.mask, &param.rectangles);
    let entropy = to_entropy(&probabilities).to_value_color_pairs(&ColorMap::Magma);
    let probabilities = probabilities.to_value_color_pairs(&ColorMap::Viridis);

    HttpResponse::Ok().json(ResponseMessage {
        probabilities,
        entropy,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Position;
    use actix_web::{App, http::StatusCode, test};
    use serde_json;

    #[actix_web::test]
    async fn test_validate_grid_size() {
        let valid_grid = Grid::new(MAX_GRID_ROWS, MAX_GRID_COLS, false);
        let invalid_grid = Grid::new(MAX_GRID_ROWS + 1, MAX_GRID_COLS + 1, false);

        assert!(validate_grid_size(&valid_grid));
        assert!(!validate_grid_size(&invalid_grid));
    }

    #[actix_web::test]
    async fn test_estimate_invalid_grid_size() {
        let mut app = test::init_service(App::new().service(estimate)).await;
        let req = test::TestRequest::post()
            .uri("/estimate")
            .set_json(&RequestParams {
                mask: Grid::new(MAX_GRID_ROWS + 1, MAX_GRID_COLS + 1, false),
                rectangles: vec![Rectangle::new(1, 1)],
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_estimate_no_rectangles() {
        let mut app = test::init_service(App::new().service(estimate)).await;
        let req = test::TestRequest::post()
            .uri("/estimate")
            .set_json(&RequestParams {
                mask: Grid::new(3, 3, false),
                rectangles: vec![],
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let response_body: ResponseMessage = test::read_body_json(resp).await;
        for y in 0..3 {
            for x in 0..3 {
                let pos = Position::new(x, y);
                assert_eq!(response_body.probabilities[&pos].0, 0.0);
                assert_eq!(response_body.entropy[&pos].0, 0.0);
            }
        }
    }

    #[actix_web::test]
    async fn test_estimate() {
        let mut app = test::init_service(App::new().service(estimate)).await;
        let req = test::TestRequest::post()
            .uri("/estimate")
            .set_json(&RequestParams {
                mask: Grid::new(3, 3, false),
                rectangles: vec![Rectangle::new(1, 1), Rectangle::new(2, 1)],
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let message: ResponseMessage = serde_json::from_slice(&body).unwrap();
        assert_eq!(message.probabilities.rows(), 3);
        assert_eq!(message.probabilities.cols(), 3);
        assert_eq!(message.entropy.rows(), 3);
        assert_eq!(message.entropy.cols(), 3);

        for y in 0..message.probabilities.rows() {
            for x in 0..message.probabilities.cols() {
                let pos = Position::new(x, y);
                assert!(
                    message.probabilities[&pos].0 >= 0.0 && message.probabilities[&pos].0 <= 1.0
                );
                assert!(message.entropy[&pos].0 >= 0.0 && message.entropy[&pos].0 <= 1.0);
            }
        }
    }
}
