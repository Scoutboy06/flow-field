#![allow(dead_code)]

use crate::vector2d::Vector2D;
use rand::{thread_rng, Rng};

// #[derive(Debug)]
pub struct FlowField {
    pub tiles: Vec<Vector2D>,
    pub points: Vec<Vector2D>,
    pub num_cols: usize,
    pub num_rows: usize,
}

impl FlowField {
    pub fn new(num_rows: usize, num_cols: usize, num_points: usize) -> Self {
        // let mut tiles: Vec<Vector2D> = Vec::with_capacity(num_cols * num_rows);
        let mut tiles: Vec<Vector2D> = vec![Vector2D::new(0.0, 0.0); num_cols * num_rows];
        let mut points: Vec<Vector2D> = Vec::with_capacity(num_points);
        let mut rng = thread_rng();

        for _ in 0..num_points {
            points.push(Vector2D::new(
                rng.gen_range(0.0..(num_rows as f32)),
                rng.gen_range(0.0..(num_rows as f32)),
            ));
        }

        for row in 0..num_rows {
            for col in 0..num_rows {
                let mut closest_point: &Vector2D = &points[0];
                let mut min_dist = f32::MAX;

                for point in points.iter() {
                    let dist = point.dist(&Vector2D::new(col as f32, row as f32));

                    if dist < min_dist {
                        min_dist = dist;
                        closest_point = point;
                    }
                }

                // The vector that will be added to the flow field
                let mut flow_vec = closest_point.copy();
                flow_vec.normalize();
                tiles[row * num_cols + col] = flow_vec;
                // tiles.push(flow_vec);
            }
        }

        Self {
            tiles,
            points,
            num_cols,
            num_rows,
        }
    }

    pub fn from_coords(&self, row: usize, col: usize) -> Option<&Vector2D> {
        self.tiles.get(row * self.num_cols + col)
    }
}
