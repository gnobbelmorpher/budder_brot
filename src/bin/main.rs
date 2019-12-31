extern crate image;

use image::{ImageBuffer, Rgb, RgbImage};
use ndarray::{azip, Array, Array2, Ix1, Ix2};
use num::complex::Complex;
use std::convert::TryInto;
use std::f64;
use std::time;

const WIDTH: u32 = 10000;
const HEIGHT: u32 = 8000;

fn render() {
    let mut mandel: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let mut budder: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    mandel_budder_brot(&mut mandel, &mut budder, 150);

    mandel.save("mandel.png").unwrap();
    budder.save("budder.png").unwrap();
}

struct RunningPixel {
    c: Complex<f64>,
    z: Complex<f64>,
}

enum Iteration {
    Running(RunningPixel),
    StoppedAtIter(u64),
}

fn mandel_budder_brot(mandel: &mut RgbImage, budder: &mut RgbImage, iters: usize) {
    let width = mandel.width();
    let height = mandel.height();
    let width_size: usize = width.try_into().unwrap();
    let height_size: usize = height.try_into().unwrap();

    let locwidth = 3f64;
    let center = Complex::new(-0.5, 0.0);
    let scale = locwidth / width as f64;
    let mut tick = 0;

    let half_width: f64 = width as f64 / 2.0;
    let half_height: f64 = height as f64 / 2.0;
    let mut values = Array::from_shape_fn((height_size, width_size), |(y, x)| {
        let r = (x as f64 - half_width) * scale + center.re;
        let i = (y as f64 - half_height) * scale + center.im;
        let c = Complex::new(r, i);
        let z = Complex::new(0.0, 0.0);
        Iteration::Running(RunningPixel { c, z })
    });

    let mut visits = Array2::<f64>::zeros((height_size, width_size));
    let start_time = time::Instant::now();

    for i in 0..iters {
        let iter_start_time = time::Instant::now();
        let mut pixels2d = mandel
            .pixels_mut()
            .collect::<Array<&mut Rgb<u8>, Ix1>>()
            .into_shape((height_size, width_size))
            .unwrap();
        azip!((p in &mut pixels2d, it in &mut values) {
            *it = match it {
                Iteration::Running(RunningPixel { ref c, ref z }) => {
                    if z.norm_sqr() > 128.0 {
                        let r = 0 + (tick * 10) as u8;
                        let g = 0 + (tick * 15) as u8;
                        let b = 255 - (tick * 10) as u8;
                        **p = Rgb([r, g, b]);
                        Iteration::StoppedAtIter(tick)
                    } else {
                        let z_new = z * z + c;
                        let x = (((z_new - center).re / scale).floor() + half_width) as usize;
                        let y = (-((z_new - center).im / scale).floor() + half_height) as usize;
                        // let x : usize = (x as i64).try_into().unwrap();
                        // let y : usize = (y as i64).try_into().unwrap();
                        if x < width_size && y < height_size {
                            visits[(y, x)] += 1f64;
                        }



                        Iteration::Running(RunningPixel { c: *c, z: z_new })
                    }
                }
                Iteration::StoppedAtIter(s) => Iteration::StoppedAtIter(*s),
            };
        });
        tick += 1;

        let ela = iter_start_time.elapsed();
        println!("Iteration {} finished after {:?}", i, ela);
    }

    let ela = start_time.elapsed();
    println!("\nfinished after {:?}", ela);

    let maxvisit = visits.fold(0f64, |b, a| b.max(*a));
    println!("maxvisit: {}", maxvisit);
    visits = visits.map(|a| a / maxvisit);
    //visits = visits.map(|a| (a - 1.0).exp());

    let mut pixels2d = budder
        .pixels_mut()
        .collect::<Array<&mut Rgb<u8>, Ix1>>()
        .into_shape((height_size, width_size))
        .unwrap();
    azip!((p in &mut pixels2d, vis in &mut visits){
        let grey = (255f64 * (*vis).log10()) as u8;
        **p = Rgb([grey, grey, grey]);
    });
}

fn main() {
    render();
}
