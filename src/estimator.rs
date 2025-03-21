use crate::types::{Grid, Position, Rectangle};
use rand::{prelude::*, rng};
use rayon::prelude::*;
use std::sync::{Arc, RwLock};

/// The number of simulations to run.
const SIMULATIONS: usize = 100000;

/// Finds all masked positions in a grid.
///
/// # Arguments
///
/// * `rect_mask` - A grid mask where `true` indicates a unmasked position and `false` indicates an masked position.
///
/// # Returns
///
/// A vector of [`Position`]s that are masked in the grid.
fn find_masked_positions(rect_mask: &Grid<bool>) -> Vec<Position> {
    let mut positions = Vec::with_capacity(rect_mask.rows() * rect_mask.cols());
    for y in 0..rect_mask.rows() {
        for x in 0..rect_mask.cols() {
            let pos = Position::new(x, y);
            if !rect_mask[&pos] {
                positions.push(pos);
            }
        }
    }
    positions
}

/// Filters out positions that are not valid for placing a rectangle within a grid.
///
/// A position is not valid if the rectangle would extend outside the grid.
/// The rectangle is allowed to rotate.
///
/// # Arguments
///
/// * `positions` - The positions to filter.
/// * `rect` - The rectangle to place.
/// * `grid_size` - The size of the grid.
///
/// # Returns
///
/// A vector of positions that are valid for placing the rectangle within the grid.
fn filter_positions(
    positions: Vec<Position>,
    rect: &Rectangle,
    grid_size: (usize, usize),
) -> Vec<Position> {
    positions
        .into_iter()
        .filter(|pos| {
            (pos.x() + rect.width() <= grid_size.0 && pos.y() + rect.height() <= grid_size.1)
                || (pos.x() + rect.height() <= grid_size.0 && pos.y() + rect.width() <= grid_size.1)
        })
        .collect()
}

/// Places rectangles within a grid.
///
/// # Arguments
///
/// * `rect_mask` - A grid mask.
/// * `rectangles` - The rectangles to be placed.
///
/// # Returns
///
/// If all the rectangles were placed, returns a grid of the placed rectangles (0 for empty and rect_id for the rectangle).
/// Otherwise, returns None.
fn place_rectangles(
    mut rect_mask: Grid<bool>,
    mut rectangles: Vec<Rectangle>,
) -> Option<Grid<usize>> {
    let mut positions = Grid::new(rect_mask.rows(), rect_mask.cols(), 0);
    let mut rng = rng();

    for (rect_idx, rect) in rectangles.iter_mut().enumerate() {
        let mut placed = false;

        // Find the positions where the rectangle may be placed.
        let unmasked_positions = find_masked_positions(&rect_mask);
        let mut filtered_positions = filter_positions(
            unmasked_positions,
            rect,
            (rect_mask.cols(), rect_mask.rows()),
        );
        if filtered_positions.is_empty() {
            return None;
        }
        filtered_positions.shuffle(&mut rng);

        // Try to place the rectangle at each position.
        for sample_pos in &filtered_positions {
            // Try to place the rectangle in two rotations.
            for _ in 0..2 {
                let y = sample_pos.y();
                let x = sample_pos.x();

                if rect_mask.all(&Position::new(x, y), rect, &false) {
                    for i in 0..rect.height() {
                        for j in 0..rect.width() {
                            let pos = Position::new(x + j, y + i);
                            rect_mask[&pos] = true;
                            positions[&pos] = rect_idx + 1;
                        }
                    }
                    placed = true;

                    break;
                }
                rect.transpose();
            }
            if placed {
                break;
            }
        }
        if !placed {
            return None;
        }
    }
    Some(positions)
}

/// Estimates the probabilities of a grid from its rectangles.
///
/// # Arguments
///
/// * `rect_mask` - A grid mask of rectangles.
/// * `rectangles` - The rectangles to be placed.
pub fn estimate_probabilities(rect_mask: &Grid<bool>, rectangles: &[Rectangle]) -> Grid<f64> {
    let prob_matrix = Arc::new(RwLock::new(Grid::new(
        rect_mask.rows(),
        rect_mask.cols(),
        0.0,
    )));
    let all_placed_count = Arc::new(RwLock::new(0));

    // Sort the rectangles by area in descending order.
    let mut rectangles = rectangles.to_owned();
    rectangles.sort_by_key(|b| std::cmp::Reverse(b.area()));

    // Run the simulation in parallel.
    (0..SIMULATIONS).into_par_iter().for_each(|_| {
        let result = place_rectangles(rect_mask.clone(), rectangles.clone());

        if let Some(result) = &result {
            *all_placed_count.write().unwrap() += 1;
            let mut matrix = prob_matrix.write().unwrap();

            for i in 0..rect_mask.rows() {
                for j in 0..rect_mask.cols() {
                    let pos = Position::new(j, i);
                    matrix[&pos] += if result[&pos] > 0 { 1.0 } else { 0.0 };
                }
            }
        }
    });

    let all_placed_count: i64 = *all_placed_count.read().unwrap();
    prob_matrix.read().unwrap().clone() / (all_placed_count as f64 + f64::EPSILON)
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
        ];
        let result = place_rectangles(rect_mask, rectangles.clone());
        assert!(result.is_some());
        let result = result.unwrap();
        let mut count = 0;
        for y in 0..result.rows() {
            for x in 0..result.cols() {
                let pos = Position::new(x, y);
                if result[&pos] > 0 {
                    count += 1;
                }
            }
        }
        assert!(count == rectangles.len());
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
        assert!(result.is_none());
    }

    #[test]
    fn test_estimate_probabilities() {
        let mut rect_mask = Grid::new(5, 9, false);
        rect_mask[&Position::new(4, 2)] = true;
        rect_mask[&Position::new(1, 0)] = true;
        rect_mask[&Position::new(8, 4)] = true;
        rect_mask[&Position::new(6, 1)] = true;
        rect_mask[&Position::new(2, 3)] = true;
        rect_mask[&Position::new(2, 4)] = true;
        rect_mask[&Position::new(3, 4)] = true;
        rect_mask[&Position::new(4, 4)] = true;
        rect_mask[&Position::new(5, 4)] = true;
        rect_mask[&Position::new(3, 2)] = true;

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
