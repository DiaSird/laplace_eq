/*-------------------------------------------------
Solution of the 2D Laplace equation
Reference:
https://www.ohmsha.co.jp/book/9784274221705/
-------------------------------------------------*/
use laplace_eq::{module::*, Config, NX, NY, LIM};
use std::f64::consts::PI;
use std::{time::Instant, *};

pub fn main() -> anyhow::Result<()> {
    env_logger::init();
    let start = Instant::now();

    let Config {
        dir_path,
        file_path,
        dir_img,
        img_path,
        img_path2,
        // } = Config::new(Arg);
    } = Config::default();

    // Initialize
    let mut u: [[f64; NX]; NY] = [[0.0; NX]; NY];

    for j in 0..NY {
        // u_0,j = sin(2πj/100)
        let temp = (2.0 * PI * j as f64 / NY as f64).sin();
        u[0][j] = temp;
    }

    // Jacobi method
    for _i in 0..LIM {
        u = iteration::iteration(u);
    }

    // Output
    // println!("{:?}", u);
    match out_csv::write_csv(&dir_path, &file_path, u) {
        Ok(ok) => ok,
        Err(err) => anyhow::bail!("{}", err),
    };

    // gif test
    // draw_pdf::plotters().unwrap();

    // drawing
    match gnuplot::write_png(&dir_img, &img_path, &file_path) {
        Ok(ok) => ok,
        Err(err) => anyhow::bail!("{}", err),
    };
    plot::plot(&dir_img, &img_path2, u)?;
    // plot_gif::plot(dir_img, u).unwrap();

    // CPU time
    cpu_time::time(start);

    Ok(())
}
