use ndarray::{
    azip, par_azip, s, Array, ArrayBase, ArrayViewMut, Axis, Data, Dimension, Ix2, Ix3, RawData,
};
use num::complex::Complex;
use sdl2::event::Event;

use butterbrot::{util, Config, Image, Inputs};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn mandelbrot() -> impl FnMut(Image, &Inputs) {
    struct RunningPixel {
        c: Complex<f64>,
        z: Complex<f64>,
    }

    enum Iteration {
        Running(RunningPixel),
        StoppedAtIter(u64),
    }

    let mut center_r = -0.5;
    let mut center_i = 0.0;
    let mut scale = 3.0 / WIDTH as f64;
    let mut tick = 0;

    let initialize = move || {
        const HALF_WIDTH: f64 = WIDTH as f64 / 2.0;
        const HALF_HEIGHT: f64 = HEIGHT as f64 / 2.0;
        Array::from_shape_fn((HEIGHT, WIDTH), |(y, x)| {
            let r = (x as f64 - HALF_WIDTH) * scale + center_r;
            let i = (y as f64 - HALF_HEIGHT) * scale + center_i;
            let c = Complex::new(r, i);
            let z = Complex::new(0.0, 0.0);
            Iteration::Running(RunningPixel { c, z })
        })
    };
    let mut values = initialize();
    move |mut image: Image, inputs| {
        for event in inputs.events.iter() {
            match event {
                Event::MouseButtonDown { .. } => values = initialize(),
                _ => {}
            }
        }
        azip!((mut pixel in image.arr.genrows_mut(), it in &mut values) {
            *it = match it {
                Iteration::Running(RunningPixel { ref c, ref z }) => {
                    if z.norm_sqr() > 16.0 {
                        pixel[0] = 0 + (tick * 10) as u8;
                        pixel[1] = 255 - (tick * 20) as u8;
                        pixel[2] = 255 - (tick * 10) as u8;
                        Iteration::StoppedAtIter(tick)
                    } else {
                        let z_new = z * z + c;
                        pixel[1] = 0;
                        pixel[2] = 0;
                        Iteration::Running(RunningPixel { c: *c, z: z_new })
                    }
                }
                Iteration::StoppedAtIter(s) => Iteration::StoppedAtIter(*s),
            };
        });
        tick += 1;
    }
}

pub fn main() {
    let closure = mandelbrot();
    let config = Config {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        print_framerate: true,
    };
    match butterbrot::run(config, closure) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
