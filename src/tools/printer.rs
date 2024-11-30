use crate::benchmarks::traits::Benchmark;
use plotters::prelude::*;

pub struct Printer<T>
where
    T: Benchmark,
{
    problem: T,
}

impl<T> Printer<T>
where
    T: Benchmark,
{
    pub fn new(problem: T) -> Self {
        Self { problem }
    }
}

impl<T> Printer<T>
where
    T: Benchmark,
{
    pub fn print2d(&self) {
        let path = format!("out/{} 2d.png", T::FUNCTION_NAME);
        let data: Vec<(f32, f32)> = (self.problem.get_min()..self.problem.get_max())
            .step(0.1)
            .values()
            .map(|x| (x, self.problem.cost_function(&[x])))
            .collect();
        let result_min = data
            .clone()
            .iter()
            .map(|(_, y)| y)
            .min_by(|a, b| a.total_cmp(b))
            .unwrap()
            .to_owned();
        let result_max = data
            .clone()
            .iter()
            .map(|(_, y)| y)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap()
            .to_owned();
        let drawing_area = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&drawing_area)
            .caption(T::FUNCTION_NAME, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                self.problem.get_min()..self.problem.get_max(),
                result_min..result_max,
            )
            .unwrap();

        chart.configure_mesh().draw().unwrap();
        chart
            .draw_series(LineSeries::new(data, &RED))
            .unwrap();

        //chart
            //.configure_series_labels()
            //.background_style(&WHITE.mix(0.8))
            // .border_style(&BLACK)
            //.draw()
            //.unwrap();

        drawing_area.present();
    }

    pub fn print3d(&self, density: f64, pitch:f64, color_th:f32) {
        let path = format!("out/{} 3d.png", T::FUNCTION_NAME);
        let data: Vec<f32> = (self.problem.get_min()..self.problem.get_max())
            .step(0.1)
            .values()
            .map(|x| {
                (self.problem.get_min()..self.problem.get_max())
                    .step(0.1)
                    .values()
                    .map(move |y| {
                        let input_arr = [x, y];
                        self.problem.cost_function(&input_arr)
                    })
                    .collect::<Vec<f32>>()
            })
            .flatten()
            .collect();
        let result_min = data
            .clone()
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap()
            .to_owned();
        let result_max = data
            .clone()
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap()
            .to_owned();
        let drawing_area = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&drawing_area)
            .caption(T::FUNCTION_NAME, ("arial", 50).into_font())
            .margin(5)
            .build_cartesian_3d(
                self.problem.get_min()..self.problem.get_max(),
                result_min..result_max,
                self.problem.get_min()..self.problem.get_max(),
            )
            .unwrap();

        chart.with_projection(|mut pb| {
            pb.pitch = pitch;
            pb.yaw = 0.5;
            pb.scale = 0.7;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()
            .unwrap();
        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (self.problem.get_min()..self.problem.get_max())
                        .step(0.1)
                        .values(),
                    (self.problem.get_min()..self.problem.get_max())
                        .step(0.1)
                        .values(),
                    |x, z| self.problem.cost_function(&[x, z]),
                )
                // .style(BLUE.mix(density).filled()),
                .style_func(&|&v| (VulcanoHSL::get_color(v / color_th).mix(density)).into())
            )
            .unwrap();

        //chart
            //.configure_series_labels()
            //.background_style(&WHITE.mix(0.8))
            // .border_style(&BLACK)
            //.draw()
            //.unwrap();

        drawing_area.present();
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use super::*;
    use crate::benchmarks::{
        ackley::Ackley, alpine2::Alpine2, deb1::Deb1, foth_dejong::FothDejong, fst_dejong::FstDeJong, griewank::Griewank, michalewicz::Michalewich, periodic::Periodic, qing::Qing, quintic::Quintic, rastrigin::Rastrigin, salomon::Salomon, schwefel::Schwefel, styblinsky_and_tang::StyblinskyAndTang, traits::HasBuilder, trd_dejong::TrdDejong, xinsheyang::XinSheYang
    };

    #[test]
    fn fstdejong() {
        let problem = FstDeJong::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(1)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.01,1.0,15.0);
    }

    #[test]
    fn schwefel() {
        let schwefel = Schwefel::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(1)
            .build()
            .unwrap();
        let sch_printer = Printer::new(schwefel);
        sch_printer.print2d();
        sch_printer.print3d(0.05,1.0,480.0);
    }

    #[test]
    fn michalewicz() {
        let michalewicz = Michalewich::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(2)
            .build()
            .unwrap()
            .set_m(10);
        let mich_printer = Printer::new(michalewicz);
        mich_printer.print2d();
        mich_printer.print3d(0.05,0.5,0.1);
    }

    #[test]
    fn rastrigin() {
        // pitch 1.0
        let rastrigin = Rastrigin::builder()
            .minimum(-4f32)
            .maximum(4f32)
            .dimensions(2)
            .build()
            .unwrap();
        let ras_printer = Printer::new(rastrigin);
        ras_printer.print2d();
        ras_printer.print3d(0.30,1.0,15.0);
    }

    #[ignore]
    #[test]
    fn trd_dejong() {
        // pitch 1.0
        let trd_dejong = TrdDejong::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(trd_dejong);
        printer.print2d();
        printer.print3d(0.05,1.0,15.0);
    }

    #[test]
    fn trd_dejong_zoom1() {
        // pitch 1.0
        let trd_dejong = TrdDejong::builder()
            .minimum(-4f32)
            .maximum(4f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(trd_dejong);
        printer.print2d();
        printer.print3d(0.8,1.0,30.0);
    }

    #[test]
    fn griewank_zoom1() {
        // pitch 1.0
        let problem = Griewank::builder()
            .minimum(-10f32)
            .maximum(10f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.5,0.5,1.0);
    }

    #[ignore]
    #[test]
    fn griewank() {
        // pitch 1.0
        let problem = Griewank::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.15,10.0,15.0);
    }

    #[ignore]
    #[test]
    fn styblinsky_and_tang() {
        // pitch 1.0
        let problem = StyblinskyAndTang::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.15,1.0,15.0);
    }

    #[test]
    fn styblinsky_and_tang_zoom1() {
        // pitch 1.0
        let problem = StyblinskyAndTang::builder()
            .minimum(-5f32)
            .maximum(5f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.50,1.0,30.0);
    }

    #[test]
    fn ackley() {
        // pitch 0.5
        let problem = Ackley::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(2)
            .build()
            .unwrap()
            .set_a(20)
            .set_b(0.2)
            .set_c(2f32 * PI);
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.05,0.5,15.0);
    }

    #[test]
    fn alpine2() {
        // pitch 1.0
        let problem = Alpine2::builder()
            .minimum(0f32)
            .maximum(10f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.5,1.0,15.0);
    }

    #[test]
    fn foth_dejong() {
        // pitch 1.0
        let problem = FothDejong::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.05,1.0,15000000.0);
    }

    #[test]
    fn salomon() {
        // pitch 1.0
        let problem = Salomon::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.15,1.0,15.0);
    }

    #[test]
    fn periodic() {
        // pitch 1.0
        let problem = Periodic::builder()
            .minimum(-10f32)
            .maximum(10f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.3,0.5,5.0);
    }

    #[test]
    fn xin_she_yang() {
        // pitch 1.0
        // slow
        let problem = XinSheYang::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.15,1.0,15.0);
    }

     #[test]
    fn qing() {
        // pitch 1.0
        let problem = Qing::builder()
            .minimum(-2f32)
            .maximum(2f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.8,1.0,15.0);
    }

     #[test]
    fn deb1() {
        // pitch 1.0
        // slow
        let problem = Deb1::builder()
            .minimum(-1f32)
            .maximum(1f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.8,1.0,0.01);
    }

     #[test]
    fn quintic() {
        // pitch 1.0
        let problem = Quintic::builder()
            .minimum(-10f32)
            .maximum(10f32)
            .dimensions(2)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d(0.8, 1.0,150000.0);
    }
}
