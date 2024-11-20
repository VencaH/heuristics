use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};

pub trait ProblemDomain {
    type Item: PartialOrd
        + Clone
        + Debug
        + Display
        + Sub<<Self as ProblemDomain>::Item, Output = <Self as ProblemDomain>::Item>
        + Add<<Self as ProblemDomain>::Item, Output = <Self as ProblemDomain>::Item>
        + Mul<<Self as ProblemDomain>::Item, Output = <Self as ProblemDomain>::Item>
        + Mul<f32, Output = f32>
        + Into<f32>;

    fn get_minimum(&self) -> Self::Item;
    fn get_maximum(&self) -> Self::Item;
    fn get_dimensions(&self) -> usize;
    fn cost_function(&self, input: &[Self::Item]) -> Self::Item;
}

pub trait HasRandom: ProblemDomain {
    fn get_random(&self) -> Vec<<Self as ProblemDomain>::Item>;
}

pub trait HasLocal: ProblemDomain {
    // type Item;

    fn get_local_next(
        &self,
        input: &[<Self as ProblemDomain>::Item],
    ) -> Vec<<Self as ProblemDomain>::Item>;
}
