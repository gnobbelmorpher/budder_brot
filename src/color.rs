use ndarray::*;
use num::{Float, NumCast};

pub struct Image<'a> {
  pub arr: ArrayViewMut<'a, u8, Ix3>,
}

impl Image<'_> {
  // Expects input to be between 0 and 1
  pub fn set_from_grayscale<S: Data + RawData<Elem = f32>>(&mut self, src: &ArrayBase<S, Ix2>) {
    azip!((mut pix in self.arr.genrows_mut(), x in src) {
        let v = (x * 255.0) as u8;
        pix[0] = v;
        pix[1] = v;
        pix[2] = v;
    })
  }

  // Expects input to be between 0 and 1
  pub fn set_from_rgb<S: Data + RawData<Elem = f32>>(
    &mut self,
    src1: &ArrayBase<S, Ix2>,
    src3: &ArrayBase<S, Ix2>,
    src2: &ArrayBase<S, Ix2>,
  ) {
    azip!((mut pix in self.arr.genrows_mut(), r in src1, g in src2, b in src3) {
        pix[0] = (255.0 * r) as u8;
        pix[1] = (255.0 * g) as u8;
        pix[2] = (255.0 * b) as u8;
    })
  }

  // Expects input to be between 0 and 1
  pub fn set_from_hsv<S: Data + RawData<Elem = f32>>(
    &mut self,
    src1: &ArrayBase<S, Ix2>,
    src3: &ArrayBase<S, Ix2>,
    src2: &ArrayBase<S, Ix2>,
  ) {
    azip!((mut pix in self.arr.genrows_mut(), h in src1, s in src2, v in src3) {
        let hsv = palette::Hsv::new(*h * 360.0, *s, *v);
        let rgb: palette::rgb::Rgb = <palette::rgb::Rgb as palette::FromColor>::from_hsv(hsv);
        pix[0] = (255.0 * rgb.red) as u8;
        pix[1] = (255.0 * rgb.green) as u8;
        pix[2] = (255.0 * rgb.blue) as u8;
    })
  }
}

impl<'a> From<ArrayViewMut<'a, u8, Ix3>> for Image<'a> {
  fn from(arr: ArrayViewMut<'a, u8, Ix3>) -> Self {
    Self { arr }
  }
}
