use std::{
    fmt::Display,
    marker::{self, PhantomData},
};

use crate::problem_definitions::{HasLocal, HasRandom, ProblemDomain};
use rand::distributions::{Distribution, Uniform};
use rand_distr::{Distribution as DDistribution, Normal};

pub trait Benchmark {
    fn get_min(&self) -> f32;
    fn get_max(&self) -> f32;
    fn get_dim(&self) -> usize;
    fn get_expected_min(&self) -> Option<f32>;
    fn get_expected_min_coords(&self) -> Option<&[f32]>;

    fn set_min(self, value: f32) -> Self;
    fn set_max(self, value: f32) -> Self;
    fn set_dim(self, value: usize) -> Self;
    fn set_expected_min(self, value: Option<f32>) -> Self;
    fn set_expected_min_coords(self, value: Option<Vec<f32>>) -> Self;

    fn cost_function(&self, input: &[f32]) -> f32;
}

impl<T> ProblemDomain for T
where
    T: Benchmark,
{
    type Item = f32;

    fn get_minimum(&self) -> Self::Item {
        self.get_min()
    }

    fn get_maximum(&self) -> Self::Item {
        self.get_max()
    }

    fn get_dimensions(&self) -> usize {
        self.get_dim()
    }

    fn cost_function(&self, input: &[Self::Item]) -> Self::Item {
        T::cost_function(self, input)
    }
}

impl<T> HasRandom for T
where
    T: Benchmark,
{
    fn get_random(&self) -> Vec<<Self as ProblemDomain>::Item> {
        let range = Uniform::new_inclusive(self.get_min(), self.get_max());
        let mut rng = rand::thread_rng();
        range.sample_iter(&mut rng).take(self.get_dim()).collect()
    }
}

impl<T> HasLocal for T
where
    T: Benchmark,
{
    fn get_local_next(
        &self,
        input: &[<Self as ProblemDomain>::Item],
    ) -> Vec<<Self as ProblemDomain>::Item> {
        let bounds = (self.get_max() - self.get_min()) / 10f32;
        let std_dev = bounds / 6f32;
        let range = Normal::new(0f32, std_dev).unwrap();
        let mut rng = rand::thread_rng();
        input
            .into_iter()
            .map(|x| {
                let mut new_x = x + range.sample(&mut rng);
                while new_x < self.get_min() || new_x > self.get_max() {
                    new_x = x + range.sample(&mut rng)
                }
                new_x
            })
            .collect()
    }
}

pub trait HasBuilder<T>
where
    T: Benchmark + Default,
{
    fn builder() -> BenchmarkBuilder<T> {
        BenchmarkBuilder {
            min: None,
            max: None,
            dim: None,
            expected_min: None,
            expected_min_coords: None,
            _marker_t: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum BuilderError {
    NoMin,
    NoMax,
    NoDim,
}

impl Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMin => write!(f, "Error while building object: Minimum is missing"),
            Self::NoMax => write!(f, "Error while building object: Maximum is missing"),
            Self::NoDim => write!(
                f,
                "Error while building object: Number of dimensions is missing"
            ),
        }
    }
}

pub struct BenchmarkBuilder<T>
where
    T: Benchmark + Default,
{
    min: Option<f32>,
    max: Option<f32>,
    dim: Option<usize>,
    expected_min: Option<f32>,
    expected_min_coords: Option<Vec<f32>>,
    _marker_t: marker::PhantomData<T>,
}
// impl<T> BenchmarkBuilder<T>
// where
//     T: Benchmark + Default,
// {
//     fn get_default() -> T {
//         T::default()
//     }
// }

impl<T> BenchmarkBuilder<T>
where
    T: Benchmark + Default,
{
    pub fn minimum(&mut self, minimum: f32) -> &mut Self {
        self.min = Some(minimum);
        self
    }
    pub fn maximum(&mut self, maximum: f32) -> &mut Self {
        self.max = Some(maximum);
        self
    }
    pub fn dimensions(&mut self, dimensions: usize) -> &mut Self {
        self.dim = Some(dimensions);
        self
    }

    pub fn expected_min(&mut self, expected_minimum: f32) -> &mut Self {
        self.expected_min = Some(expected_minimum);
        self
    }

    pub fn expected_min_coords(&mut self, expected_minimum_coords: Vec<f32>) -> &mut Self {
        self.expected_min_coords = Some(expected_minimum_coords);
        self
    }

    pub fn build(&self) -> Result<T, BuilderError> {
        Ok(T::default()
            .set_expected_min(self.expected_min.clone())
            .set_expected_min_coords(self.expected_min_coords.clone()))
        .and_then(|mut fd| {
            if let Some(min) = self.min {
                Ok(fd.set_min(min))
            } else {
                Err(BuilderError::NoMin)
            }
        })
        .and_then(|mut fd| {
            if let Some(max) = self.max {
                Ok(fd.set_max(max))
            } else {
                Err(BuilderError::NoMax)
            }
        })
        .and_then(|mut fd| {
            if let Some(dim) = self.dim {
                Ok(fd.set_dim(dim))
            } else {
                Err(BuilderError::NoDim)
            }
        })
    }
}
