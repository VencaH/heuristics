use std::fmt::{Debug, Display};

pub trait ProblemDomain {
    type Item: PartialOrd + Clone + Debug + Display;

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
