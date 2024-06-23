use std::{
    fmt::Display,
    marker::{self, PhantomData},
};

use crate::problem_definitions::{HasLocal, HasRandom, ProblemDomain};
use rand::distributions::{Distribution, Uniform};
use rand_distr::{Distribution as DDistribution, Normal};

trait Benchmark {
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

    fn cost_function(input: &[Self::Item]) -> Self::Item {
        input.into_iter().map(|x| x * *x).sum()
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

trait HasBuilder<T>
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

struct BenchmarkBuilder<T>
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
    fn minimum(&mut self, minimum: f32) -> &mut Self {
        self.min = Some(minimum);
        self
    }
    fn maximum(&mut self, maximum: f32) -> &mut Self {
        self.max = Some(maximum);
        self
    }
    fn dimensions(&mut self, dimensions: usize) -> &mut Self {
        self.dim = Some(dimensions);
        self
    }

    fn expected_min(&mut self, expected_minimum: f32) -> &mut Self {
        self.expected_min = Some(expected_minimum);
        self
    }

    fn expected_min_coords(&mut self, expected_minimum_coords: Vec<f32>) -> &mut Self {
        self.expected_min_coords = Some(expected_minimum_coords);
        self
    }

    fn build(&self) -> Result<T, BuilderError> {
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

mod fst_dejong {
    use super::*;

    #[derive(Debug)]
    struct FstDeJong {
        min: f32,
        max: f32,
        dim: usize,
        expected_min: Option<f32>,
        expected_min_coords: Option<Vec<f32>>,
    }

    impl HasBuilder<FstDeJong> for FstDeJong {}

    impl Benchmark for FstDeJong {
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
    }

    impl Default for FstDeJong {
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
        #[test]
        fn random() {
            let dejong = FstDeJong::builder()
                .minimum(-5f32)
                .maximum(5f32)
                .dimensions(5usize)
                .build()
                .unwrap();
            let random_1 = dejong.get_random();
            let random_2 = dejong.get_random();
            let random_3 = dejong.get_random();

            assert_ne!(random_1, random_2);
            assert_ne!(random_1, random_3);
            assert_ne!(random_2, random_3);
        }

        #[test]
        fn radom_in_range() {
            let dejong = FstDeJong::builder()
                .minimum(-5f32)
                .maximum(5f32)
                .dimensions(5usize)
                .build()
                .unwrap();
            let random_1 = dejong.get_random();
            let random_2 = dejong.get_random();
            let random_3 = dejong.get_random();

            assert!(random_1.into_iter().all(|x| x > -5f32 && x < 5f32));
            assert!(random_2.into_iter().all(|x| x > -5f32 && x < 5f32));
            assert!(random_3.into_iter().all(|x| x > -5f32 && x < 5f32));
        }

        #[test]
        fn local_next() {
            let dejong = FstDeJong::builder()
                .minimum(-5f32)
                .maximum(5f32)
                .dimensions(5usize)
                .build()
                .unwrap();
            let random_1 = dejong.get_random();
            let new_local = dejong.get_local_next(&random_1);

            assert!(!new_local.is_empty());
            assert_ne!(new_local, random_1);
        }

        #[test]
        fn local_next_in_range() {
            let dejong = FstDeJong::builder()
                .minimum(-5f32)
                .maximum(5f32)
                .dimensions(5usize)
                .build()
                .unwrap();
            let random_1 = dejong.get_random();
            let new_local = dejong.get_local_next(&random_1);
            let new_low = dejong.get_local_next(&vec![-5.0, -5.0, -5.0, -5.0, -5.0]);
            let new_high = dejong.get_local_next(&vec![5.0, 5.0, 5.0, 5.0, 5.0]);

            assert!(new_local.into_iter().all(|x| x > -5f32 && x < 5f32));
            assert!(new_low.into_iter().all(|x| x > -5f32 && x < 5f32));
            assert!(new_high.into_iter().all(|x| x > -5f32 && x < 5f32));
        }
    }
}
