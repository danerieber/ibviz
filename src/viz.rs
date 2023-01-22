use opencv::{
    imgproc::{fill_poly, polylines}, prelude::Mat, core::{CV_8UC3, ToInputOutputArray, Point},
};

use crate::{convert::IbInto, piano::Piano};

pub fn frame(w: i32, h: i32, color: (f64, f64, f64)) -> Result<Mat, opencv::Error> {
    Mat::new_rows_cols_with_default(h, w, CV_8UC3, color.ib_into())
}

pub fn outline_key<T: Piano>(
    frame: &mut dyn ToInputOutputArray,
    piano: &mut T,
    key: usize,
    color: (f64, f64, f64),
) -> Result<(), opencv::Error> {
    let pts = piano.key_box(key);
    polylines(frame, &pts, true, color.ib_into(), 1, 1, 0)
}

pub fn fill_key<T: Piano>(
    frame: &mut dyn ToInputOutputArray,
    piano: &mut T,
    key: usize,
    color: (f64, f64, f64),
) -> Result<(), opencv::Error> {
    let pts = piano.key_box(key);
    fill_poly(frame, &pts, color.ib_into(), 1, 0, Point::default())
}
