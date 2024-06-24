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
