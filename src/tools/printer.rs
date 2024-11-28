use crate::{benchmarks::traits::Benchmark};
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
        let path  = format!("out/{} 2d.png", T::FUNCTION_NAME);
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
            .unwrap()
            .label(T::FUNCTION_NAME)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();

        drawing_area.present();
    }

    pub fn print3d(&self) {
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
            .caption(T::FUNCTION_NAME, ("sans-serif", 50).into_font())
            .margin(5)
            .build_cartesian_3d(
                self.problem.get_min()..self.problem.get_max(),
                result_min..result_max,
                self.problem.get_min()..self.problem.get_max(),
            )
            .unwrap();

        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.9;
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
                .style(BLUE.mix(0.01).filled()),
            )
            .unwrap()
            .label(T::FUNCTION_NAME)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();

        drawing_area.present();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::benchmarks::{fst_dejong::FstDeJong,schwefel::Schwefel, traits::HasBuilder};

    #[test]
    fn print_test_graph() {
        let problem = FstDeJong::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(1)
            .build()
            .unwrap();
        let printer = Printer::new(problem);
        printer.print2d();
        printer.print3d();

        let schwefel = Schwefel::builder()
            .minimum(-100f32)
            .maximum(100f32)
            .dimensions(1)
            .build()
            .unwrap();
        let sch_printer = Printer::new(schwefel);
        sch_printer.print2d();
        sch_printer.print3d();
    }
}
