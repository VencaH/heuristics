use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

use crate::benchmarks::traits::{Benchmark, HasBuilder};
use rand_distr::num_traits::{Float, ToPrimitive};

// source: https://www.sfu.ca/~ssurjano/ackley.html
#[derive(Debug)]
pub struct Ackley {
    min: f32,
    max: f32,
    dim: usize,
    a: i32,
    b: f32,
    c: f32,
    expected_min: Option<f32>,
    expected_min_coords: Option<Vec<f32>>,
}

impl Ackley {
    pub fn get_a(&self) -> i32 {
        self.a
    }

    pub fn get_b(&self) -> f32 {
        self.b
    }

    pub fn get_c(&self) -> f32 {
        self.c
    }

    pub fn set_a(mut self, a: i32) -> Self {
        self.a = a;
        self
    }

    pub fn set_b(mut self, b: f32) -> Self {
        self.b = b;
        self
    }

    pub fn set_c(mut self, c: f32) -> Self {
        self.c = c;
        self
    }
}

impl HasBuilder<Ackley> for Ackley {}

impl Benchmark for Ackley {
    const FUNCTION_NAME: &'static str = "Ackley function";
    fn get_min(&self) -> f32 {
        self.min
    }

    fn get_max(&self) -> f32 {
        self.max
    }

    fn get_dim(&self) -> usize {
        self.dim
    }

    fn get_expected_min(&self) -> Option<f32> {
        self.expected_min
    }

    fn get_expected_min_coords(&self) -> Option<&[f32]> {
        self.expected_min_coords.as_deref()
    }

    fn set_min(mut self, value: f32) -> Self {
        self.min = value;
        self
    }

    fn set_max(mut self, value: f32) -> Self {
        self.max = value;
        self
    }

    fn set_dim(mut self, value: usize) -> Self {
        self.dim = value;
        self
    }

    fn set_expected_min(mut self, value: Option<f32>) -> Self {
        self.expected_min = value;
        self
    }

    fn set_expected_min_coords(mut self, value: Option<Vec<f32>>) -> Self {
        self.expected_min_coords = value;
        self
    }

    fn cost_function(&self, input: &[f32]) -> f32 {
        let d = self.get_dim() as f32;
        ( - self.a as f32)
            .mul(
                (-self.b.mul(
                    input
                        .into_iter()
                        .map(|x| x.powi(2))
                        .sum::<f32>()
                        .mul(1f32.div(d))
                        .sqrt(),
                ))
                .exp(),
            )
            .sub(
                input
                    .into_iter()
                    .map(|x| self.c.mul(x).cos())
                    .sum::<f32>()
                    .mul(1f32.div(d))
                    .exp(),
            )
            .add(self.a as f32)
            .add(1f32.exp())
    }
}

impl Default for Ackley {
    fn default() -> Self {
        Self {
            min: 0f32,
            max: 0f32,
            dim: 0usize,
            a: 20,
            b: 0.2,
            c: 2f32 * PI,
            expected_min: None,
            expected_min_coords: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::problem_definitions::{HasLocal, HasRandom};

    #[test]
    fn random() {
        let ackley = Ackley::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = ackley.get_random();
        let random_2 = ackley.get_random();
        let random_3 = ackley.get_random();

        assert_ne!(random_1, random_2);
        assert_ne!(random_1, random_3);
        assert_ne!(random_2, random_3);
    }

    #[test]
    fn radom_in_range() {
        let dejong = Ackley::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = dejong.get_random();
        let random_2 = dejong.get_random();
        let random_3 = dejong.get_random();

        assert!(random_1.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(random_2.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(random_3.into_iter().all(|x| x > -500f32 && x < 500f32));
    }

    #[test]
    fn local_next() {
        let ackley = Ackley::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = ackley.get_random();
        let new_local = ackley.get_local_next(&random_1);

        assert!(!new_local.is_empty());
        assert_ne!(new_local, random_1);
    }

    #[test]
    fn local_next_in_range() {
        let ackley = Ackley::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = ackley.get_random();
        let new_local = ackley.get_local_next(&random_1);
        let new_low = ackley.get_local_next(&vec![-500.0, -500.0, -500.0, -500.0, -500.0]);
        let new_high = ackley.get_local_next(&vec![500.0, 500.0, 500.0, 500.0, 500.0]);

        assert!(new_local.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(new_low.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(new_high.into_iter().all(|x| x > -500f32 && x < 500f32));
    }
}
