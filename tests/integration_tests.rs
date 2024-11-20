use heuristics::benchmarks::{
    fst_dejong::FstDeJong, schwefel::Schwefel, snd_dejong::SndDeJong, traits::HasBuilder,
};
use heuristics::solvers::random_search::RandomSearch;

mod random_search {
    use super::*;

    #[test]
    fn fst_dejong() {
        let problem = FstDeJong::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(5)
            .build()
            .unwrap();
        let mut random = RandomSearch::new(10000, problem);
        random.run();

        assert_eq!(random.get_history().len(), 10000);
        assert!(random.get_best_cost().is_some());
    }
    #[test]
    fn snd_dejong() {
        let problem = SndDeJong::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(5)
            .build()
            .unwrap();
        let mut random = RandomSearch::new(10000, problem);
        random.run();

        assert_eq!(random.get_history().len(), 10000);
        assert!(random.get_best_cost().is_some());
    }
    #[test]
    fn schwefel() {
        let problem = Schwefel::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(10)
            .build()
            .unwrap();
        let mut random = RandomSearch::new(10000, problem);
        random.run();

        println!("best: {:?}", random.get_best_cost());
        assert_eq!(random.get_history().len(), 10000);
        assert!(random.get_best_cost().is_some());
    }
}
mod local_search {
    use heuristics::solvers::local_search::LocalSearch;

    use super::*;

    #[test]
    fn fst_dejong() {
        let problem = FstDeJong::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(5)
            .build()
            .unwrap();
        let mut local = LocalSearch::new(10, problem);
        local.run();

        assert!(local.get_best_cost().is_some());
    }
    #[test]
    fn snd_dejong() {
        let problem = SndDeJong::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(5)
            .build()
            .unwrap();
        let mut local = LocalSearch::new(10, problem);
        local.run();

        assert!(local.get_best_cost().is_some());
    }
    #[test]
    fn schwefel() {
        let problem = Schwefel::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(10)
            .build()
            .unwrap();
        let mut local = LocalSearch::new(10, problem);
        local.run();

        println!("best: {:?}", local.get_best_cost());
        assert!(local.get_best_cost().is_some());
    }
}

mod hill_climber {
    use heuristics::solvers::hill_climber::HillClimber;

    use super::*;

    #[test]
    fn fst_dejong() {
        let problem = FstDeJong::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(5)
            .build()
            .unwrap();
        let mut hill_climber = HillClimber::new(1000, 10, problem);
        hill_climber.run();

        assert!(hill_climber.get_best_cost().is_some());
    }
    #[test]
    fn snd_dejong() {
        let problem = SndDeJong::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(5)
            .build()
            .unwrap();
        let mut hill_climber = HillClimber::new(1000, 10, problem);
        hill_climber.run();

        assert!(hill_climber.get_best_cost().is_some());
    }
    #[test]
    fn schwefel() {
        let problem = Schwefel::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(10)
            .build()
            .unwrap();
        let mut hill_climber = HillClimber::new(1000, 10, problem);
        hill_climber.run();

        println!("best: {:?}", hill_climber.get_best_cost());
        assert!(hill_climber.get_best_cost().is_some());
    }
}

mod simulated_annealing {
    use heuristics::solvers::simulated_annealing::SimulatedAnnealing;

    use super::*;

    #[test]
    fn fst_dejong() {
        let problem = FstDeJong::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(5)
            .build()
            .unwrap();
        let mut simulated_annealing = SimulatedAnnealing::new(10, 1000f32, 0.1, 0.998, problem);
        simulated_annealing.run();

        assert!(simulated_annealing.get_best_cost().is_some());
    }
    #[test]
    fn snd_dejong() {
        let problem = SndDeJong::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(5)
            .build()
            .unwrap();
        let mut simulated_annealing = SimulatedAnnealing::new(10, 1000f32, 0.1, 0.998, problem);
        simulated_annealing.run();

        assert!(simulated_annealing.get_best_cost().is_some());
    }
    #[test]
    fn schwefel() {
        let problem = Schwefel::builder()
            .minimum(-500f32)
            .maximum(500f32)
            .dimensions(10)
            .build()
            .unwrap();
        let mut simulated_annealing = SimulatedAnnealing::new(10, 1000f32, 0.1, 0.998, problem);
        simulated_annealing.run();

        println!("best: {:?}", simulated_annealing.get_best_cost());
        assert!(simulated_annealing.get_best_cost().is_some());
    }
}

mod de_rnd_1_bin {
    use super::*;
    use heuristics::evol_arg::de::{De, Strategy, Variant};

    #[test]
    fn fst_dejong() {
        let problem = FstDeJong::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(20usize)
            .build()
            .unwrap();
        let mut de_rnd_1_bin = De::new(Variant::Rnd, 1, Strategy::Bin, 4000, 10, 0.8, 0.5, problem);
        de_rnd_1_bin.run();
        println!("best: {:?}", de_rnd_1_bin.get_best());
        assert!(de_rnd_1_bin.get_best().is_some());
    }

    #[test]
    fn schwefel() {
        let problem = Schwefel::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(20usize)
            .build()
            .unwrap();
        let mut de_rnd_1_bin = De::new(Variant::Rnd, 1, Strategy::Bin, 4000, 10, 0.8, 0.9, problem);
        de_rnd_1_bin.run();
        println!("best: {:?}", de_rnd_1_bin.get_best());
        assert!(de_rnd_1_bin.get_best().is_some());
        assert_eq!(de_rnd_1_bin.get_cost_function_evaluations(), 40000);
    }
}
