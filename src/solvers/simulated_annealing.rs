use std::f32::consts::E;

use rand_distr::{Distribution, Uniform};

use crate::problem_definitions::{HasLocal, HasRandom, ProblemDomain};

pub struct SimulatedAnnealing<T>
where
    T: ProblemDomain + HasLocal + HasRandom,
{
    max_local_iter: i32,
    min_temp: f32,
    step: f32,
    current_temp: f32,
    current_best: Option<T::Item>,
    current_best_coords: Option<Vec<T::Item>>,
    cost_history: Vec<T::Item>,
    problem: T,
}

impl<T> SimulatedAnnealing<T>
where
    T: ProblemDomain + HasRandom + HasLocal,
{
    pub fn new(max_local_iter: i32, max_temp: f32, min_temp: f32, step: f32, problem: T) -> Self {
        SimulatedAnnealing {
            max_local_iter,
            min_temp,
            step,
            current_temp: max_temp,
            current_best: None,
            current_best_coords: None,
            cost_history: vec![],
            problem,
        }
    }

    pub fn run(&mut self) -> () {
        let start_input = self.problem.get_random();
        let start_cost = self.problem.cost_function(&start_input);
        let mut current_best = start_cost.clone();
        let mut current_best_coords = start_input;
        self.cost_history.push(start_cost);

        while self.current_temp >= self.min_temp {
            for _ in 0..self.max_local_iter {
                let local_coords = self.problem.get_local_next(&current_best_coords);
                let local_cost = self.problem.cost_function(&local_coords);
                let metro_result = self.metropolis(local_cost.clone());
                if metro_result == 1 {
                    current_best = local_cost.clone();
                    current_best_coords = local_coords;
                }
                self.cost_history.push(current_best.clone());
            }
            self.current_temp = self.current_temp * self.step;
        }
        self.current_best = Some(current_best);
        self.current_best_coords = Some(current_best_coords);
    }

    fn metropolis(&self, new_cost: T::Item) -> i32 {
        if let (Some(current_best), Some(_)) =
            (self.current_best.clone(), self.current_best_coords.clone())
        {
            let difference =
                Into::<f32>::into(new_cost.clone()) - Into::<f32>::into(current_best.clone());
            if difference < 0f32 {
                1
            } else {
                let probability = 1f32 / E.powf(difference / self.current_temp);
                let roll = Uniform::new(0f32, 1f32).sample(&mut (rand::thread_rng()));
                if roll < probability {
                    1
                } else {
                    -1
                }
            }
        } else {
            1
        }
    }

    pub fn get_history(&self) -> &[T::Item] {
        &self.cost_history
    }

    pub fn get_best_cost(&self) -> Option<T::Item> {
        self.current_best.clone()
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
        MockProblem {}
        impl ProblemDomain for MockProblem{
             type Item = f32;
            fn get_minimum(&self) -> f32;
            fn get_maximum(&self) -> f32;
            fn get_dimensions(&self) -> usize;
            fn cost_function(&self, input: &[f32]) -> f32;

        }

        impl HasRandom for MockProblem{
            fn get_random(&self) -> Vec<f32>;
        }

        impl HasLocal for MockProblem {
            fn get_local_next(&self, input: &[f32]) -> Vec<f32>;
        }
    }

    #[test]
    fn run() {
        let mut mocked_problem = MockMockProblem::new();
        mocked_problem.expect_get_random().returning(|| {
            let range = Uniform::new_inclusive(-500f32, 500f32);
            let mut rng = rand::thread_rng();
            range.sample_iter(&mut rng).take(5).collect()
        });
        mocked_problem
            .expect_get_local_next()
            .times(10..)
            .returning(|_| {
                let range = Uniform::new_inclusive(-500f32, 500f32);
                let mut rng = rand::thread_rng();
                range.sample_iter(&mut rng).take(5).collect()
            });
        mocked_problem.expect_cost_function().returning(|_| {
            let range = Uniform::new_inclusive(0f32, 15000f32);
            let mut rng = rand::thread_rng();
            let ret = range.sample(&mut rng);
            ret
        });
        let mut sa = SimulatedAnnealing::new(10, 1000f32, 0.1, 0.98, mocked_problem);
        sa.run();
        assert_ne!(sa.cost_history.len(), 0);
    }

    //     #[test]
    //     fn get_0() {
    //         let range = Uniform::new(1usize, 1000usize);
    //         let fst_part = range.sample(&mut (rand::thread_rng()));
    //         let snd_part = 1000 - fst_part - 1;
    //         let mut seq_rnd = Sequence::new();
    //         let mut seq_cf = Sequence::new();
    //         let mut mocked_problem = MockMockProblem::new();

    //         // first part - random results
    //         mocked_problem
    //             .expect_get_random()
    //             .times(fst_part)
    //             .in_sequence(&mut seq_rnd)
    //             .returning(|| {
    //                 let range = Uniform::new_inclusive(-500f32, 500f32);
    //                 let mut rng = rand::thread_rng();
    //                 range.sample_iter(&mut rng).take(5).collect()
    //             });
    //         mocked_problem
    //             .expect_cost_function()
    //             .times(fst_part)
    //             .in_sequence(&mut seq_cf)
    //             .returning(|_| {
    //                 let range = Uniform::new_inclusive(0f32, 15000f32);
    //                 let mut rng = rand::thread_rng();
    //                 range.sample(&mut rng)
    //             });

    //         //middle part - specific results which we can check later
    //         mocked_problem
    //             .expect_get_random()
    //             .times(1)
    //             .in_sequence(&mut seq_rnd)
    //             .returning(|| vec![0f32, 0f32, 0f32, 0f32, 0f32]);
    //         mocked_problem
    //             .expect_cost_function()
    //             .times(1)
    //             .in_sequence(&mut seq_cf)
    //             .returning(|_| 0f32);

    //         //last part to fill iterations with random numbers
    //         mocked_problem
    //             .expect_get_random()
    //             .times(snd_part)
    //             .in_sequence(&mut seq_rnd)
    //             .returning(|| {
    //                 let range = Uniform::new_inclusive(-500f32, 500f32);
    //                 let mut rng = rand::thread_rng();
    //                 range.sample_iter(&mut rng).take(5).collect()
    //             });
    //         mocked_problem
    //             .expect_cost_function()
    //             .times(snd_part)
    //             .in_sequence(&mut seq_cf)
    //             .returning(|_| {
    //                 let range = Uniform::new_inclusive(0f32, 15000f32);
    //                 let mut rng = rand::thread_rng();
    //                 range.sample(&mut rng)
    //             });
    //         let mut random_search = RandomSearch::new(1000, mocked_problem);
    //         random_search.run();
    //         assert_eq!(random_search.current_best, Some(0f32));
    //         assert_eq!(
    //             random_search.current_best_coords,
    //             Some(vec![0f32, 0f32, 0f32, 0f32, 0f32])
    //         );
    //     }
}
