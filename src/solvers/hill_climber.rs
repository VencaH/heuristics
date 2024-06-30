use crate::problem_definitions::{HasLocal, HasRandom, ProblemDomain};

pub struct HillClimber<T>
where
    T: ProblemDomain + HasLocal + HasRandom,
{
    max_iter: i32,
    max_local_iter: i32,
    curren_best: Option<T::Item>,
    curren_best_coords: Option<Vec<T::Item>>,
    cost_history: Vec<T::Item>,
    problem: T,
}

impl<T> HillClimber<T>
where
    T: ProblemDomain + HasRandom + HasLocal,
{
    pub fn new(max_iter: i32, max_local_iter: i32, problem: T) -> Self {
        HillClimber {
            max_iter,
            max_local_iter,
            curren_best: None,
            curren_best_coords: None,
            cost_history: vec![],
            problem,
        }
    }

    pub fn run(&mut self) -> () {
        let start_input = self.problem.get_random();
        let start_cost = self.problem.cost_function(&start_input);
        let mut current_best = start_cost.clone();
        let mut current_best_coords = start_input.clone();
        let mut current_coodrs = start_input;
        self.cost_history.push(start_cost);

        for _ in 0..self.max_iter {
            let (local_best, local_best_coords) = self.evaluate_local(&current_best_coords);
            if local_best > current_best {
                current_best = local_best.clone();
                current_best_coords = local_best_coords.clone();
            }
            current_coodrs = local_best_coords;
            self.cost_history.push(local_best);
        }
        self.curren_best = Some(current_best);
        self.curren_best_coords = Some(current_best_coords);
    }

    fn evaluate_local(&mut self, input: &[T::Item]) -> (T::Item, Vec<T::Item>) {
        let mut local_best_coords = self.problem.get_local_next(input);
        let mut local_best = self.problem.cost_function(&local_best_coords);

        for _ in 1..self.max_local_iter {
            let new_local_coords = self.problem.get_local_next(input);
            let new_local = self.problem.cost_function(&new_local_coords);
            if new_local < local_best {
                local_best = new_local;
                local_best_coords = new_local_coords;
            }
        }
        (local_best, local_best_coords)
    }

    pub fn get_history(&self) -> &[T::Item] {
        &self.cost_history
    }

    pub fn get_best_cost(&self) -> Option<T::Item> {
        self.curren_best.clone()
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
        let mut hill_climber = HillClimber::new(1000, 10, mocked_problem);
        hill_climber.run();
        assert_ne!(hill_climber.cost_history.len(), 0);
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
