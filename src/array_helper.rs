use ndarray::*;

pub fn neg_to_pos_one_fixed_ratio(width: usize, height: usize) -> Array<f64, Ix3> {
    // TODO: Try producing this with linspace and broadcast instead
    let half_larger_size = width.max(height) as f64 / 2.0;
    let half_width = width as f64 / 2.0;
    let half_height = height as f64 / 2.0;
    Array::from_shape_fn((height, width, 2), |(y, x, z)| {
        if z == 0 {
            (x as f64 - half_width) / half_larger_size
        } else {
            (y as f64 - half_height) / half_larger_size
        }
    })
}

pub fn zero_to_one_fixed_ratio(width: usize, height: usize) -> Array<f64, Ix3> {
    // TODO: Try producing this with linspace and broadcast instead
    let larger_size = width.max(height) as f64;
    Array::from_shape_fn((height, width, 2), |(y, x, z)| match z {
        0 => x as f64 / larger_size,
        1 => (height - y) as f64 / larger_size,
        _ => panic!(),
    })
}
