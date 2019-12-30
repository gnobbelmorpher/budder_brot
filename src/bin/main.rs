extern crate image;

use image::{ImageBuffer, Rgb, RgbImage};
use ndarray::{azip, Array, Ix1, Ix2};
use num::complex::Complex;
use std::convert::TryInto;

const WIDTH: u32 = 8000;
const HEIGHT: u32 = 4500;

fn render() {
  let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

  mandelbrot(&mut img);

  img.save("test.png").unwrap();
}

fn mandelbrot(img: &mut RgbImage) {
  let width: usize = img.width().try_into().unwrap();
  let height: usize = img.height().try_into().unwrap();

  struct RunningPixel {
    c: Complex<f64>,
    z: Complex<f64>,
  }

  enum Iteration {
    Running(RunningPixel),
    StoppedAtIter(u64),
  }

  let center_r = -0.5;
  let center_i = 0.0;
  let scale = 3.0 / WIDTH as f64;
  let mut tick = 0;

  let initialize = move || {
    const HALF_WIDTH: f64 = WIDTH as f64 / 2.0;
    const HALF_HEIGHT: f64 = HEIGHT as f64 / 2.0;
    Array::from_shape_fn((height, width), |(y, x)| {
      let r = (x as f64 - HALF_WIDTH) * scale + center_r;
      let i = (y as f64 - HALF_HEIGHT) * scale + center_i;
      let c = Complex::new(r, i);
      let z = Complex::new(0.0, 0.0);
      Iteration::Running(RunningPixel { c, z })
    })
  };
  let mut values = initialize();

  for i in 0..200 {
    println!("Iteration {}", i);
    let pixels1d: Array<&mut Rgb<u8>, Ix1> = img.pixels_mut().collect();
    let mut pixels2d: Array<&mut Rgb<u8>, Ix2> = pixels1d.into_shape((height, width)).unwrap();
    azip!((p in &mut pixels2d, it in &mut values) {
        *it = match it {
            Iteration::Running(RunningPixel { ref c, ref z }) => {
                if z.norm_sqr() > 16.0 {
                    let r = 0 + (tick * 10) as u8;
                    let g = 255 - (tick * 20) as u8;
                    let b = 255 - (tick * 10) as u8;
                    **p = Rgb([r, r, r]);
                    Iteration::StoppedAtIter(tick)
                } else {
                    let z_new = z * z + c;
                    Iteration::Running(RunningPixel { c: *c, z: z_new })
                }
            }
            Iteration::StoppedAtIter(s) => Iteration::StoppedAtIter(*s),
        };
    });
    tick += 1;
  }
}

fn main() {
  render();
}
