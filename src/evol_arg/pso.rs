use crate::problem_definitions::{HasRandom, ProblemDomain};
use rand::{random, seq::SliceRandom};
#[derive(Debug)]
pub struct Particle<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    pub current_cost: f32,
    pub best_coords: usize,
    best_cost: f32,
    pub current_coordinates: usize,
    pub coordinates_history: Vec<Vec<T::Item>>,
    pub velocity: Vec<T::Item>,
}

impl<T> Clone for Particle<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    fn clone(&self) -> Self {
        Self {
            current_cost: self.current_cost,
            best_cost: self.best_cost,
            best_coords: self.best_coords,
            current_coordinates: self.current_coordinates,
            coordinates_history: self.coordinates_history.clone(),
            velocity: self.velocity.clone(),
        }
    }
}

impl<T> Particle<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    pub fn new(cost: f32, coordinates: Vec<T::Item>, velocity: Vec<T::Item>) -> Self {
        Self {
            current_cost: cost,
            best_cost: cost,
            best_coords: 0usize,
            coordinates_history: vec![coordinates],
            current_coordinates: 0usize,
            velocity,
        }
    }

    fn update_particle(&mut self, coordinates: Vec<T::Item>, velocity: Vec<T::Item>, cost: f32) {
        self.coordinates_history.push(coordinates);
        self.velocity = velocity;
        let len = self.coordinates_history.len();
        self.current_coordinates += 1;
        self.update_cost(cost);
    }

    fn update_cost(&mut self, cost: f32) {
        self.current_cost = cost;
        if cost < self.best_cost {
            self.best_cost = cost;
            self.best_coords = self.current_coordinates;
        }
    }
}

pub struct Pso<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    max_cf: i32,
    population_size: usize,
    inertia_weight: f32,
    personal_priority: f32,
    social_priority: f32,

    current_best: Option<T::Item>,
    current_best_coordinates: Option<Vec<T::Item>>,
    particles: Vec<Particle<T>>,
    cost_function_evaluations: i32,

    problem: T,
}

impl<T> Pso<T>
where
    T: ProblemDomain<Item = f32> + HasRandom,
{
    pub fn new(
        max_cf: i32,
        population_size: usize,
        inertia_weight: f32,
        personal_priority: f32,
        social_priority: f32,
        problem: T,
    ) -> Self {
        Self {
            max_cf,
            population_size,
            inertia_weight,
            personal_priority,
            social_priority,
            current_best: None,
            current_best_coordinates: None,
            particles: Vec::new(),
            cost_function_evaluations: 0,
            problem,
        }
    }

    pub fn run(&mut self) {
        let new_pop: Vec<Particle<T>> = (0..self.population_size)
            .into_iter()
            .map(|_| {
                let coords = self.problem.get_random();
                let velocity = self.problem.get_random();
                let cost = self.run_cost_fn(&coords);

                Particle::new(cost, coords, velocity)
            })
            .collect();
        self.particles = new_pop.clone();
        self.update_best();

        while self.cost_function_evaluations < self.max_cf {
            self.particles = self
                .particles
                .clone()
                .iter()
                .map(|particle| self.move_particle(particle))
                .collect();
            self.update_best();
        }
    }

    pub fn get_best(&self) -> Option<f32> {
        self.current_best
    }

    pub fn get_cost_function_evaluations(&self) -> i32 {
        self.cost_function_evaluations
    }

    fn run_cost_fn(&mut self, input: &[T::Item]) -> f32 {
        self.cost_function_evaluations += 1;
        self.problem.cost_function(input)
    }

    fn get_current_gen_best(&self) -> (f32, Vec<T::Item>) {
        let best = self
            .get_particles()
            .iter()
            .max_by(|a, b| a.current_cost.partial_cmp(&b.current_cost).unwrap())
            .unwrap();
        (
            best.current_cost,
            best.coordinates_history[best.current_coordinates].clone(),
        )
    }

    fn get_particles(&self) -> &[Particle<T>] {
        &self.particles
    }

    fn update_best(&mut self) {
        let (current_gen_best, current_gen_best_coords) = self.get_current_gen_best();
        if let Some(current_best) = self.current_best {
            if current_gen_best < current_best {
                self.current_best = Some(current_gen_best);
                self.current_best_coordinates = Some(current_gen_best_coords);
            }
        } else {
            self.current_best = Some(current_gen_best);
            self.current_best_coordinates = Some(current_gen_best_coords);
        }
    }

    fn move_particle(&mut self, particle: &Particle<T>) -> Particle<T> {
        let mut particle = particle.clone();
        let weighted_velocity = particle
            .velocity
            .clone()
            .into_iter()
            .map(|a| a * self.inertia_weight);
        let personal_velocity = particle.coordinates_history[particle.best_coords]
            .clone()
            .into_iter()
            .zip(particle.coordinates_history[particle.current_coordinates].iter())
            .map(|(a, b)| a - b)
            .zip(
                (0..self.problem.get_dimensions())
                    .into_iter()
                    .map(|_| random::<f32>()),
            )
            .map(|(a, b)| a * b)
            .map(|a| a * self.personal_priority);

        let social_velocity = self
            .current_best_coordinates
            .as_ref()
            .unwrap()
            .into_iter()
            .zip(particle.coordinates_history[particle.current_coordinates].iter())
            .map(|(a, b)| a - b)
            .zip(
                (0..self.problem.get_dimensions())
                    .into_iter()
                    .map(|_| random::<f32>()),
            )
            .map(|(a, b)| a * b)
            .map(|a| a * self.personal_priority);
        let new_velocity: Vec<f32> = weighted_velocity
            .zip(social_velocity)
            .map(|(a, b)| a + b)
            .zip(personal_velocity)
            .map(|(a, b)| a + b)
            .collect();
        let new_coords: Vec<T::Item> = particle.coordinates_history[particle.current_coordinates]
            .iter()
            .zip(new_velocity.iter())
            .map(|(a, b)| a + b)
            .collect();
        let new_cost = self.run_cost_fn(&new_coords);
        particle.update_particle(new_coords, new_velocity, new_cost);
        particle
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
        let expected_calls = 500;
        let mut mocked_problem = MockProblem::new();
        mocked_problem.expect_get_dimensions().returning(|| 3usize);
        mocked_problem
            .expect_get_random()
            .times(20) // in pso twice per particle during inicialization
            .returning(|| vec![1.0, 2.0, 3.0]);
        mocked_problem
            .expect_cost_function()
            .times(expected_calls)
            .returning(|_| {
                let range = Uniform::new_inclusive(0f32, 15000f32);
                let mut rng = rand::thread_rng();
                range.sample(&mut rng)
            });

        let mut pso = Pso::new(500, 10, 0.7, 0.8, 0.9, mocked_problem);
        pso.run();
        assert!(pso.current_best_coordinates.is_some());
        assert_eq!(pso.cost_function_evaluations, expected_calls as i32);
    }
}
