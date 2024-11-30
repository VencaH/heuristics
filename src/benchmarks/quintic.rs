use std::{
    f32::consts::{E, PI},
    ops::{Add, Div, Mul, Sub},
};

use crate::benchmarks::traits::{Benchmark, HasBuilder};
use rand::random;
use rand_distr::num_traits::{Float, ToPrimitive};

// source:  https://al-roomi.org/benchmarks/unconstrained/n-dimensions/231-deb-s-function-no-01
//
#[derive(Debug)]
pub struct Quintic {
    min: f32,
    max: f32,
    dim: usize,
    expected_min: Option<f32>,
    expected_min_coords: Option<Vec<f32>>,
}

impl HasBuilder<Quintic> for Quintic {}

impl Benchmark for Quintic {
    const FUNCTION_NAME: &'static str = "Quintic function";
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
        input
            .iter()
            .map(|&x| {
                x.powi(5)
                    .sub(3f32.mul(x.powi(4)))
                    .add(4f32.mul(x.powi(3)))
                    .add(2f32.mul(x.powi(2)))
                    .sub(10f32.mul(x))
                    .sub(4f32)
                    .abs()
            })
            .sum::<f32>()
    }
}

impl Default for Quintic {
    fn default() -> Self {
        Self {
            min: 0f32,
            max: 0f32,
            dim: 0usize,
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
        let quintic = Quintic::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = quintic.get_random();
        let random_2 = quintic.get_random();
        let random_3 = quintic.get_random();

        assert_ne!(random_1, random_2);
        assert_ne!(random_1, random_3);
        assert_ne!(random_2, random_3);
    }

    #[test]
    fn radom_in_range() {
        let dejong = Quintic::builder()
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
        let quintic = Quintic::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = quintic.get_random();
        let new_local = quintic.get_local_next(&random_1);

        assert!(!new_local.is_empty());
        assert_ne!(new_local, random_1);
    }

    #[test]
    fn local_next_in_range() {
        let quintic = Quintic::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = quintic.get_random();
        let new_local = quintic.get_local_next(&random_1);
        let new_low = quintic.get_local_next(&vec![-500.0, -500.0, -500.0, -500.0, -500.0]);
        let new_high = quintic.get_local_next(&vec![500.0, 500.0, 500.0, 500.0, 500.0]);

        assert!(new_local.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(new_low.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(new_high.into_iter().all(|x| x > -500f32 && x < 500f32));
    }
}
