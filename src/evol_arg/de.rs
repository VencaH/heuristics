use crate::problem_definitions::{HasRandom, ProblemDomain};
use rand::seq::SliceRandom;

pub enum Variant {
    Rnd,
    Best,
}

pub enum Strategy {
    Bin,
}

#[derive(Debug)]
pub struct Member<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    cost: f32,
    coordinates: Vec<T::Item>,
}

impl<T> Clone for Member<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    fn clone(&self) -> Self {
        Self {
            cost: self.cost.clone(),
            coordinates: self.coordinates.clone(),
        }
    }
}

impl<T> Member<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    pub fn new(cost: f32, coordinates: Vec<T::Item>) -> Self {
        Self { cost, coordinates }
    }

    pub fn get_coordinates(&self) -> &[T::Item] {
        &self.coordinates
    }

    pub fn get_cost(&self) -> f32 {
        self.cost
    }
}

pub struct De<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    // DE parameters
    max_generations: i32,
    population_size: usize,
    scaling_factor: T::Item,
    crossover_probability: f32,
    difference_vectors: i32,
    variant: Variant,
    strategy: Strategy,

    // results
    current_best: Option<Member<T>>,
    generations_history: Vec<Vec<Member<T>>>,
    current_generation: usize,
    cost_function_evaluations: i32,

    problem: T,
}

impl<T> De<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    pub fn new(
        variant: Variant,
        difference_vectors: i32,
        strategy: Strategy,
        max_generations: i32,
        population_size: usize,
        scaling_factor: f32,
        crossover_probability: f32,
        problem: T,
    ) -> Self {
        Self {
            max_generations,
            population_size,
            scaling_factor,
            crossover_probability,
            difference_vectors,
            variant,
            strategy,
            current_best: None,
            generations_history: Vec::new(),
            current_generation: 0usize,
            cost_function_evaluations: 0,
            problem,
        }
    }

    pub fn run(&mut self) -> () {
        let new_gen = self.get_random_generation();
        self.add_new_generation(new_gen);
        self.update_best();
        for _ in 1..self.max_generations {
            let new_generation = self.generations_history[self.current_generation]
                .clone()
                .into_iter()
                .enumerate()
                .map(|(index, member)| self.mutate(&member, index))
                .collect();
            self.add_new_generation(new_generation);
            self.update_best();
        }
    }

    fn run_cost_fn(&mut self, input: &[T::Item]) -> f32 {
        self.cost_function_evaluations += 1;
        self.problem.cost_function(input).into()
    }

    fn get_current_gen_best(&self) -> Member<T> {
        self.get_current_generation()
            .iter()
            .max_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap())
            .unwrap()
            .clone()
    }

    fn update_best(&mut self) -> () {
        let current_gen_best = self.get_current_gen_best();
        if let Some(member) = self.current_best.clone() {
            if member.cost > current_gen_best.cost {
                self.current_best = Some(current_gen_best);
            }
        } else {
            self.current_best = Some(current_gen_best);
        }
    }

    fn get_random_generation(&mut self) -> Vec<Member<T>> {
        (0..self.population_size)
            .into_iter()
            .map(|_| {
                let coords = self.problem.get_random();
                Member {
                    cost: self.run_cost_fn(&coords),
                    coordinates: coords,
                }
            })
            .collect()
    }

    fn get_current_generation(&self) -> &[Member<T>] {
        &self.generations_history[self.current_generation]
    }

    fn get_mut_current_generation(&mut self) -> &mut [Member<T>] {
        &mut self.generations_history[self.current_generation]
    }

    fn add_new_generation(&mut self, new_generation: Vec<Member<T>>) -> () {
        if self.generations_history.len() != 0 {
            self.current_generation += 1;
        }
        self.generations_history.push(new_generation);
    }

    pub fn get_best(&self) -> Option<Member<T>> {
        self.current_best.clone()
    }

    pub fn get_cost_function_evaluations(&self) -> i32 {
        self.cost_function_evaluations
    }

    fn mutate(&mut self, member: &Member<T>, index: usize) -> Member<T> {
        let mut bias = 0f32;
        let bias_increment =
            (self.crossover_probability + 0.001) / self.problem.get_dimensions() as f32;
        if self.difference_vectors != 1 {
            todo!()
        }
        let mut current_gen = self
            .get_current_generation()
            .iter()
            .enumerate()
            .filter(|(a, _)| *a != index)
            .map(|(_, member)| member.to_owned())
            .collect::<Vec<Member<T>>>();
        let mut selected_vectors = current_gen.choose_multiple(&mut rand::thread_rng(), 3);

        let trial_vector = selected_vectors
            .next()
            .unwrap()
            .coordinates
            .iter()
            .zip(selected_vectors.next().unwrap().coordinates.iter())
            .map(|(a, b)| (a.clone() - b.clone()) * self.scaling_factor.clone())
            .zip(match self.variant {
                Variant::Rnd => selected_vectors
                    .next()
                    .unwrap()
                    .coordinates
                    .clone()
                    .into_iter(),
                Variant::Best => self.get_current_gen_best().coordinates.into_iter(),
            })
            .map(|(a, b)| a.clone() + b.clone())
            .zip(member.coordinates.iter())
            .map(|(a, b)| {
                if rand::random::<f32>() + bias > self.crossover_probability {
                    bias = 0f32;
                    *b
                } else {
                    bias += bias_increment;
                    a
                }
            })
            .collect::<Vec<T::Item>>();
        let cost = self.run_cost_fn(&trial_vector).into();
        if cost < member.cost {
            Member {
                cost,
                coordinates: trial_vector,
            }
        } else {
            member.to_owned()
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use rand::distributions::Uniform;
    use rand_distr::Distribution;

    mock! {
        Problem {}
        impl ProblemDomain for Problem {
             type Item = f32;
             fn get_minimum(&self) -> f32;
             fn get_maximum(&self) -> f32;
             fn get_dimensions(&self) -> usize;
             fn cost_function(&self, input : &[f32]) -> f32;
        }

        impl HasRandom for Problem {
            fn get_random(&self) -> Vec<f32>;
        }
    }

    #[test]
    fn expected_cost_calls() {
        let expected_calls = 5000;
        let mut mocked_problem = MockProblem::new();
        mocked_problem.expect_get_dimensions().returning(|| 3usize);
        mocked_problem
            .expect_get_random()
            .times(10)
            .returning(|| vec![1.0, 2.0, 3.0]);
        mocked_problem
            .expect_cost_function()
            .times(expected_calls)
            .returning(|_| {
                let range = Uniform::new_inclusive(0f32, 15000f32);
                let mut rng = rand::thread_rng();
                range.sample(&mut rng)
            });

        let mut de_rng_1_bin = De::new(
            Variant::Rnd,
            1,
            Strategy::Bin,
            500,
            10,
            0.8,
            0.5,
            mocked_problem,
        );
        de_rng_1_bin.run();
        assert!(de_rng_1_bin.current_best.is_some());
        assert_eq!(de_rng_1_bin.cost_function_evaluations, 5000);
        assert_eq!(de_rng_1_bin.generations_history.len(), 500);
        assert_eq!(de_rng_1_bin.generations_history[0].len(), 10);
    }
}
