use opencv::{
    core::{Point, ToInputOutputArray, Vector, CV_8UC3},
    imgproc::{fill_poly, polylines},
    prelude::Mat,
};

use crate::{
    convert::IbInto,
    piano::{Piano, Piano3D},
};

pub fn frame(w: i32, h: i32, color: (f64, f64, f64)) -> Result<Mat, opencv::Error> {
    Mat::new_rows_cols_with_default(h, w, CV_8UC3, color.ib_into())
}

pub fn outline_key<T: Piano>(
    frame: &mut dyn ToInputOutputArray,
    piano: &mut T,
    key: usize,
    color: (f64, f64, f64),
) -> Result<(), opencv::Error> {
    let pts: Vector<Point> = piano.get_vector(key).ib_into();
    polylines(frame, &pts, true, color.ib_into(), 1, 1, 0)
}

pub fn fill_key<T: Piano>(
    frame: &mut dyn ToInputOutputArray,
    piano: &mut T,
    key: usize,
    color: (f64, f64, f64),
) -> Result<(), opencv::Error> {
    let pts: Vector<Point> = piano.get_vector(key).ib_into();
    fill_poly(frame, &pts, color.ib_into(), 1, 0, Point::default())
}

pub fn fill_key_scale_height(
    frame: &mut dyn ToInputOutputArray,
    piano: &mut Piano3D,
    key: usize,
    scale_height: f64,
    color: (f64, f64, f64),
) -> Result<(), opencv::Error> {
    let mut ib_rect = piano.piano2d.get_ib_rect(key);
    let height = ib_rect.bl.y - ib_rect.tl.y;
    let extend_height = height * (scale_height - 1.0) / 2.0;
    let ty = ib_rect.tl.y - extend_height;
    let by = ib_rect.bl.y + extend_height;
    ib_rect.tl.y = ty;
    ib_rect.tr.y = ty;
    ib_rect.br.y = by;
    ib_rect.bl.y = by;
    let pts: Vector<Point> = piano.perspective_transform(&ib_rect.ib_into()).ib_into();
    fill_poly(frame, &pts, color.ib_into(), 1, 0, Point::default())
}
