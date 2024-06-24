use crate::problem_definitions::{HasRandom, ProblemDomain};

pub struct RandomSearch<T>
where
    T: ProblemDomain + HasRandom,
{
    max_iter: i32,
    current_best: Option<T::Item>,
    current_best_coords: Option<Vec<T::Item>>,
    cost_history: Vec<T::Item>,
    problem: T,
}

impl<T> RandomSearch<T>
where
    T: ProblemDomain + HasRandom,
{
    pub fn new(max_iter: i32, problem: T) -> Self {
        RandomSearch {
            max_iter,
            current_best: None,
            current_best_coords: None,
            cost_history: vec![],
            problem,
        }
    }

    pub fn run(&mut self) -> () {
        let start_input = self.problem.get_random();
        let start_cost = self.problem.cost_function(&start_input);
        self.current_best = Some(start_cost.clone());
        self.current_best_coords = Some(start_input);
        self.cost_history.push(start_cost);

        for _ in 1..self.max_iter {
            let local_input = self.problem.get_random();
            let local_cost = self.problem.cost_function(&local_input);
            match &mut self.current_best {
                None => {
                    self.current_best = Some(local_cost.clone());
                    self.current_best_coords = Some(local_input);
                }
                Some(best) => {
                    if &local_cost < best {
                        self.current_best = Some(local_cost.clone());
                        self.current_best_coords = Some(local_input);
                    }
                }
            }
            self.cost_history.push(local_cost);
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
    }}

    #[test]
    fn expected_iters() {
        let mut mocked_problem = MockMockProblem::new();
        mocked_problem
            .expect_get_random()
            .times(1000)
            .returning(|| {
                let range = Uniform::new_inclusive(-500f32, 500f32);
                let mut rng = rand::thread_rng();
                range.sample_iter(&mut rng).take(5).collect()
            });
        mocked_problem
            .expect_cost_function()
            .times(1000)
            .returning(|_| {
                let range = Uniform::new_inclusive(0f32, 15000f32);
                let mut rng = rand::thread_rng();
                range.sample(&mut rng)
            });
        let mut random_search = RandomSearch::new(1000, mocked_problem);
        random_search.run();
        assert_eq!(random_search.cost_history.len(), 1000);
    }

    #[test]
    fn get_0() {
        let range = Uniform::new(1usize, 1000usize);
        let fst_part = range.sample(&mut (rand::thread_rng()));
        let snd_part = 1000 - fst_part - 1;
        let mut seq_rnd = Sequence::new();
        let mut seq_cf = Sequence::new();
        let mut mocked_problem = MockMockProblem::new();

        // first part - random results
        mocked_problem
            .expect_get_random()
            .times(fst_part)
            .in_sequence(&mut seq_rnd)
            .returning(|| {
                let range = Uniform::new_inclusive(-500f32, 500f32);
                let mut rng = rand::thread_rng();
                range.sample_iter(&mut rng).take(5).collect()
            });
        mocked_problem
            .expect_cost_function()
            .times(fst_part)
            .in_sequence(&mut seq_cf)
            .returning(|_| {
                let range = Uniform::new_inclusive(0f32, 15000f32);
                let mut rng = rand::thread_rng();
                range.sample(&mut rng)
            });

        //middle part - specific results which we can check later
        mocked_problem
            .expect_get_random()
            .times(1)
            .in_sequence(&mut seq_rnd)
            .returning(|| vec![0f32, 0f32, 0f32, 0f32, 0f32]);
        mocked_problem
            .expect_cost_function()
            .times(1)
            .in_sequence(&mut seq_cf)
            .returning(|_| 0f32);

        //last part to fill iterations with random numbers
        mocked_problem
            .expect_get_random()
            .times(snd_part)
            .in_sequence(&mut seq_rnd)
            .returning(|| {
                let range = Uniform::new_inclusive(-500f32, 500f32);
                let mut rng = rand::thread_rng();
                range.sample_iter(&mut rng).take(5).collect()
            });
        mocked_problem
            .expect_cost_function()
            .times(snd_part)
            .in_sequence(&mut seq_cf)
            .returning(|_| {
                let range = Uniform::new_inclusive(0f32, 15000f32);
                let mut rng = rand::thread_rng();
                range.sample(&mut rng)
            });
        let mut random_search = RandomSearch::new(1000, mocked_problem);
        random_search.run();
        assert_eq!(random_search.current_best, Some(0f32));
        assert_eq!(
            random_search.current_best_coords,
            Some(vec![0f32, 0f32, 0f32, 0f32, 0f32])
        );
    }
}
