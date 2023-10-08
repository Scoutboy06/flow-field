#![allow(dead_code)]

use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_angle(angle: f32) -> Self {
        let x = angle.cos();
        let y = angle.sin();

        Self::new(x, y)
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);

        Self::new(x, y)
    }

    pub fn copy(&self) -> Self {
        Self::new(self.x, self.y)
    }

    pub fn add(&mut self, other: &Self) -> Self {
        self.x += other.x;
        self.y += other.y;

        Self::new(self.x, self.y)
    }

    pub fn sub(&mut self, other: &Self) -> Self {
        self.x -= other.x;
        self.y -= other.x;

        Self::new(self.x, self.y)
    }

    pub fn mult(&mut self, factor: f32) -> Self {
        self.x *= factor;
        self.y *= factor;

        Self::new(self.x, self.y)
    }

    pub fn div(&mut self, factor: f32) -> Self {
        self.x /= factor;
        self.y /= factor;

        Self::new(self.x, self.y)
    }

    /**  Normalize the vector to length 1 (make it a unit vector). */
    pub fn normalize(&mut self) {
        let length = self.mag();
        self.x /= length;
        self.y /= length;
    }

    /** Calculates the magnitude (length) of the vector and returns the result as a float. (This is simply the equation sqrt(x*x + y*y).) */
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /** Calculates the squared magnitude of the vector and returns the result as a float. (This is simply the equation `x*x + y*y`.) Faster if the real length is not required in the case of comparing vectors, etc. */
    pub fn mag_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /** Calculates the Euclidean distance between two points (considering a point as a vector object). */
    pub fn dist(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn limit(&mut self, max: f32) {
        self.normalize();
        self.mult(max);
    }
}
