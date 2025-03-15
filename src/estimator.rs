use std::sync::{Arc, RwLock};

use crate::types::{Grid, Position, Rectangle};
use rand::{prelude::*, rng};
use rayon::prelude::*;

/// The number of simulations to run.
const SIMULATIONS: usize = 100000;

/// Places rectangles within a grid.
///
/// # Arguments
///
/// * `rect_mask` - A grid mask.
/// * `rectangles` - The rectangles to be placed.
fn place_rectangles(mut rect_mask: Grid<bool>, mut rectangles: Vec<Rectangle>) -> Grid<usize> {
    let mut positions = Grid::new(rect_mask.rows(), rect_mask.cols(), 0);

    let mut rng = rng();
    rectangles.shuffle(&mut rng);

    for mut rect in rectangles {
        for _ in 0..2 {
            let y = rng.random_range(0..=rect_mask.rows().saturating_sub(rect.height()));
            let x = rng.random_range(0..=rect_mask.cols().saturating_sub(rect.width()));

            if rect_mask.all(&Position::new(x, y), &rect, &false) {
                for i in 0..rect.height() {
                    for j in 0..rect.width() {
                        let pos = Position::new(x + j, y + i);
                        rect_mask[&pos] = true;
                        positions[&pos] += 1;
                    }
                }
                break;
            }
            rect.transpose();
        }
    }
    positions
}

/// Estimates the probabilities of a grid from its rectangles.
///
/// # Arguments
///
/// * `rect_mask` - A grid mask of rectangles.
/// * `rectangles` - The rectangles to be placed.
pub fn estimate_probabilities(rect_mask: &Grid<bool>, rectangles: &Vec<Rectangle>) -> Grid<f64> {
    let prob_matrix = Arc::new(RwLock::new(Grid::new(
        rect_mask.rows(),
        rect_mask.cols(),
        0.0,
    )));

    (0..SIMULATIONS).into_par_iter().for_each(|_| {
        let result = place_rectangles(rect_mask.clone(), rectangles.clone());
        let mut matrix = prob_matrix.write().unwrap();

        for i in 0..rect_mask.rows() {
            for j in 0..rect_mask.cols() {
                let pos = Position::new(j, i);
                matrix[&pos] += result[&pos] as f64;
            }
        }
    });

    prob_matrix.read().unwrap().clone() / SIMULATIONS as f64
}

/// Computes the entropy of a grid from its probabilities.
pub fn to_entropy(probabilities: &Grid<f64>) -> Grid<f64> {
    let mut entropy = Grid::new(probabilities.rows(), probabilities.cols(), 0.0);
    for i in 0..probabilities.rows() {
        for j in 0..probabilities.cols() {
            let pos = Position::new(j, i);
            let p = probabilities[&pos];
            entropy[&pos] = (-p * (p + f64::EPSILON).log2()
                - (1.0 - p) * (1.0 - p + f64::EPSILON).log2())
            .clamp(0.0, 1.0);
        }
    }
    entropy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_rectangles_all_placed() {
        let rect_mask = Grid::new(5, 9, false);
        let rectangles = vec![
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
        ];
        let result = place_rectangles(rect_mask, rectangles);
        let mut count = 0;
        for y in 0..result.rows() {
            for x in 0..result.cols() {
                let pos = Position::new(x, y);
                if result[&pos] > 0 {
                    count += 1;
                }
            }
        }
        assert!(count > 0);
    }

    #[test]
    fn test_place_rectangles_no_placed() {
        let mut rect_mask = Grid::new(5, 9, false);
        for y in 0..rect_mask.rows() {
            for x in 0..rect_mask.cols() {
                let pos = Position::new(x, y);
                rect_mask[&pos] = true;
            }
        }
        let rectangles = vec![
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
            Rectangle::new(1, 1),
        ];
        let result = place_rectangles(rect_mask, rectangles);
        let mut count = 0;
        for y in 0..result.rows() {
            for x in 0..result.cols() {
                let pos = Position::new(x, y);
                if result[&pos] > 0 {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 0);
    }

    #[test]
    fn test_estimate_probabilities() {
        let rect_mask = Grid::new(5, 9, false);
        let rectangles = vec![
            Rectangle::new(2, 1),
            Rectangle::new(2, 1),
            Rectangle::new(2, 1),
            Rectangle::new(2, 1),
            Rectangle::new(2, 1),
            Rectangle::new(2, 1),
            Rectangle::new(3, 1),
            Rectangle::new(3, 1),
            Rectangle::new(3, 1),
            Rectangle::new(3, 1),
            Rectangle::new(4, 1),
            Rectangle::new(4, 1),
        ];
        let probabilities = estimate_probabilities(&rect_mask, &rectangles);
        assert_eq!(probabilities.rows(), rect_mask.rows());
        assert_eq!(probabilities.cols(), rect_mask.cols());
        // Check if all values are between 0 and 1
        for y in 0..rect_mask.rows() {
            for x in 0..rect_mask.cols() {
                let pos = Position::new(x, y);
                assert!(probabilities[&pos] >= 0.0 && probabilities[&pos] <= 1.0);
            }
        }
    }

    #[test]
    fn test_to_entropy() {
        let probabilities = Grid::new(5, 9, 0.5);
        let entropy = to_entropy(&probabilities);
        assert_eq!(entropy.rows(), probabilities.rows());
        assert_eq!(entropy.cols(), probabilities.cols());
        // Check if all values are between 0 and 1
        for y in 0..probabilities.rows() {
            for x in 0..probabilities.cols() {
                let pos = Position::new(x, y);
                assert!(entropy[&pos] >= 0.0 && entropy[&pos] <= 1.0);
            }
        }
    }
}
