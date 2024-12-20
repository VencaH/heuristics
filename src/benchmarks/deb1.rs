use std::{
    f32::consts:: PI,
    ops::{Div, Mul,}
};

use crate::benchmarks::traits::{Benchmark, HasBuilder};

// source:  https://al-roomi.org/benchmarks/unconstrained/n-dimensions/231-deb-s-function-no-01
//
#[derive(Debug)]
pub struct Deb1 {
    min: f32,
    max: f32,
    dim: usize,
    expected_min: Option<f32>,
    expected_min_coords: Option<Vec<f32>>,
}

impl HasBuilder<Deb1> for Deb1 {}

impl Benchmark for Deb1 {
    const FUNCTION_NAME: &'static str = "Deb's function No. 01";
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
        let d = self.dim as f32;
        -1f32
            .div(d)
            .mul(input.iter().map(|&x| 5f32.mul(PI).mul(x).sin().powi(6)).sum::<f32>())
    }
}

impl Default for Deb1 {
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
        let deb1 = Deb1::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = deb1.get_random();
        let random_2 = deb1.get_random();
        let random_3 = deb1.get_random();

        assert_ne!(random_1, random_2);
        assert_ne!(random_1, random_3);
        assert_ne!(random_2, random_3);
    }

    #[test]
    fn radom_in_range() {
        let dejong = Deb1::builder()
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
        let deb1 = Deb1::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = deb1.get_random();
        let new_local = deb1.get_local_next(&random_1);

        assert!(!new_local.is_empty());
        assert_ne!(new_local, random_1);
    }

    #[test]
    fn local_next_in_range() {
        let deb1 = Deb1::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = deb1.get_random();
        let new_local = deb1.get_local_next(&random_1);
        let new_low = deb1.get_local_next(&vec![-500.0, -500.0, -500.0, -500.0, -500.0]);
        let new_high = deb1.get_local_next(&vec![500.0, 500.0, 500.0, 500.0, 500.0]);

        assert!(new_local.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(new_low.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(new_high.into_iter().all(|x| x > -500f32 && x < 500f32));
    }
}
