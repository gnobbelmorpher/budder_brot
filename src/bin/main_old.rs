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

fn range_check<S: Data + RawData<Elem = f64>, D: Dimension>(src: &ArrayBase<S, D>) -> (f64, f64) {
    let init_min_value = std::f64::MAX;
    let init_max_value = std::f64::MIN;
    src.fold(
        (init_min_value, init_max_value),
        |(min_value, max_value), &x| {
            if x < min_value {
                (x, max_value)
            } else if x > max_value {
                (min_value, x)
            } else {
                (min_value, max_value)
            }
        },
    )
}

pub fn noisy_circle() -> impl FnMut(Image, &Inputs) {
    use noise::{NoiseFn, Seedable};
    let gen1 = noise::Fbm::new();
    let gen2 = noise::Fbm::new().set_seed(2);
    let gen3 = noise::Fbm::new().set_seed(3);
    let coords = util::neg_to_pos_one_fixed_ratio(WIDTH, HEIGHT);
    let coords_scaled = 10.0 * coords.clone();
    let coords_tup = coords_scaled.map_axis(Axis(2), |arr| (arr[0], arr[1]));
    let circle = coords_tup.mapv(|(x, y)| {
        let z = (x * x + y * y);
        (0.5 - 0.5 * z, 0.5 - 0.2 * z, 0.5 - 0.1 * z)
    });
    let mut out1 = Array::<f32, Ix2>::zeros(coords_tup.dim());
    let mut out2 = Array::<f32, Ix2>::zeros(coords_tup.dim());
    let mut out3 = Array::<f32, Ix2>::zeros(coords_tup.dim());
    let mut tick = 0;
    move |mut image, _| {
        par_azip!((o1 in &mut out1, o2 in &mut out2, o3 in &mut out3, &(x, y) in &coords_tup, &(c1, c2, c3) in &circle) {
            *o1 = (gen1.get([x, y, tick as f64 / 100.0]) * 0.5 + 0.5 + c1).min(1.0).max(0.0) as f32;
            *o2 = (gen2.get([x, y, tick as f64 / 100.0]) * 0.5 + 0.5 + c2).min(1.0).max(0.0) as f32;
            *o3 = (gen3.get([x, y, tick as f64 / 100.0]) * 0.5 + 0.5 + c3).min(1.0).max(0.0) as f32
        });
        image.set_from_rgb(&out1, &out2, &out3);
        tick += 2;
    }
}

pub fn rgb_fbm_noise() -> impl FnMut(Image, &Inputs) {
    use simdnoise::NoiseBuilder;
    let mut tick = 0;
    move |mut image: Image, _| {
        let vec1 = NoiseBuilder::fbm_3d_offset(0.0, WIDTH, 0.0, HEIGHT, tick as f32 / 1.0, 1)
            .generate_scaled(0.0, 1.0);
        let vec2 =
            NoiseBuilder::fbm_3d_offset(0.0, WIDTH, 0.0, HEIGHT, 100.0 + tick as f32 / 1.0, 1)
                .generate_scaled(0.0, 1.0);
        let vec3 =
            NoiseBuilder::fbm_3d_offset(0.0, WIDTH, 0.0, HEIGHT, 30.0 + tick as f32 / 1.0, 1)
                .generate_scaled(0.0, 1.0);
        let noise1 = Array::from(vec1).into_shape((HEIGHT, WIDTH)).unwrap();
        let noise2 = Array::from(vec2).into_shape((HEIGHT, WIDTH)).unwrap();
        let noise3 = Array::from(vec3).into_shape((HEIGHT, WIDTH)).unwrap();
        // image.set_from_grayscale(&coords.index_axis(Axis(2), 0));
        image.set_from_rgb(&noise1, &noise2, &noise3);
        tick += 2;
    }
}

pub fn rgb_simd_noise() -> impl FnMut(Image, &Inputs) {
    use simdnoise::NoiseBuilder;
    let mut tick = 0;
    move |mut image: Image, _| {
        let vec1 =
            NoiseBuilder::turbulence_3d_offset(1.0, WIDTH, 1.0, HEIGHT, tick as f32 / 1.0, 1)
                .generate_scaled(0.0, 1.0);
        let vec2 = NoiseBuilder::turbulence_3d_offset(
            1.0,
            WIDTH,
            1.0,
            HEIGHT,
            100.0 + tick as f32 / 1.0,
            1,
        )
        .generate_scaled(0.0, 1.0);
        let vec3 = NoiseBuilder::turbulence_3d_offset(
            1.0,
            WIDTH,
            1.0,
            HEIGHT,
            30.0 + tick as f32 / 1.0,
            1,
        )
        .generate_scaled(0.0, 1.0);
        let noise1 = Array::from(vec1).into_shape((HEIGHT, WIDTH)).unwrap();
        let noise2 = Array::from(vec2).into_shape((HEIGHT, WIDTH)).unwrap();
        let noise3 = Array::from(vec3).into_shape((HEIGHT, WIDTH)).unwrap();
        // image.set_from_grayscale(&coords.index_axis(Axis(2), 0));
        image.set_from_hsv(&noise1, &noise2, &noise3);
        tick += 2;
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
