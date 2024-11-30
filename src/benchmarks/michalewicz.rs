use std::{f32::consts::PI, ops::{Add, Div, Mul}};
use crate::benchmarks::traits::{Benchmark, HasBuilder};

// source:https://www.sfu.ca/~ssurjano/michal.html

#[derive(Debug)]
pub struct Michalewicz {
    min: f32,
    max: f32,
    dim: usize,
    m: i32,
    expected_min: Option<f32>,
    expected_min_coords: Option<Vec<f32>>,
}

impl Michalewicz {
    pub fn get_m(&self) -> i32 {
        self.m
    }

    pub fn set_m(mut self, m: i32) -> Self {
        self.m = m;
        self
    }
}

impl HasBuilder<Michalewicz> for Michalewicz {}

impl Benchmark for Michalewicz {
    const FUNCTION_NAME: &'static str = "Michalewicz function";
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
        println!("inputs: {:?}", input);
        let result =-input
            .iter()
            .enumerate()
            .map(|(i, x)| {
                x.sin()
                    .mul(
                        (i as f32)
                        .add(1f32)
                        .mul(x.powi(2))
                        .div(PI)
                        .sin()
                        .powi(self.m *2)
                        )
            })
            .sum::<f32>();
        println!("{result}");
        result
    }
}

impl Default for Michalewicz {
    fn default() -> Self {
        Self {
            min: 0f32,
            max: 0f32,
            m: 10,
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
        let michalewicz = Michalewicz::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = michalewicz.get_random();
        let random_2 = michalewicz.get_random();
        let random_3 = michalewicz.get_random();

        assert_ne!(random_1, random_2);
        assert_ne!(random_1, random_3);
        assert_ne!(random_2, random_3);
    }

    #[test]
    fn radom_in_range() {
        let dejong = Michalewicz::builder()
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
        let michalewicz = Michalewicz::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = michalewicz.get_random();
        let new_local = michalewicz.get_local_next(&random_1);

        assert!(!new_local.is_empty());
        assert_ne!(new_local, random_1);
    }

    #[test]
    fn local_next_in_range() {
        let michalewicz = Michalewicz::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(5usize)
            .build()
            .unwrap();
        let random_1 = michalewicz.get_random();
        let new_local = michalewicz.get_local_next(&random_1);
        let new_low = michalewicz.get_local_next(&vec![-500.0, -500.0, -500.0, -500.0, -500.0]);
        let new_high = michalewicz.get_local_next(&vec![500.0, 500.0, 500.0, 500.0, 500.0]);

        assert!(new_local.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(new_low.into_iter().all(|x| x > -500f32 && x < 500f32));
        assert!(new_high.into_iter().all(|x| x > -500f32 && x < 500f32));
    }
}
