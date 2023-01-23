use opencv::{
    core::{no_array, Point2d, Vector},
    prelude::Mat,
};

use crate::convert::*;

pub struct Piano2D {
    pub width: f64,
    pub height: f64,
    pub start_key: usize,
    pub n_keys: usize,
    key_types: Vec<bool>,
    key_type_indices: Vec<usize>,
    n_white_keys: usize,
    key_ib_rects: Vec<Option<IbRectangle>>,
}

pub struct Piano3D {
    pub border: IbRectangle,
    pub piano2d: Piano2D,
    h_matrix: Mat,
    key_ib_rects: Vec<Option<IbRectangle>>,
}

#[derive(Clone, Copy)]
pub struct IbRectangle {
    pub tl: Point2d,
    pub tr: Point2d,
    pub br: Point2d,
    pub bl: Point2d,
}

impl IbRectangle {
    pub fn new(
        top_left: (f64, f64),
        top_right: (f64, f64),
        bottom_right: (f64, f64),
        bottom_left: (f64, f64),
    ) -> Self {
        IbRectangle {
            tl: top_left.ib_into(),
            tr: top_right.ib_into(),
            br: bottom_right.ib_into(),
            bl: bottom_left.ib_into(),
        }
    }
}

pub trait Piano {
    fn get_ib_rect(&mut self, key: usize) -> IbRectangle;
    fn get_vector(&mut self, key: usize) -> Vector<Point2d>;
    fn is_black(&self, key: usize) -> bool;
}

pub const KEY_PATTERN: [bool; 12] = [
    false, true, false, false, true, false, true, false, false, true, false, true,
];

impl Piano2D {
    pub fn new(width: f64, height: f64, start_key: usize, n_keys: usize) -> Self {
        let mut key_types = vec![false; n_keys];
        let mut key_type_indices = vec![0; n_keys];
        let mut n_white_keys = 0;
        let mut n_black_keys = 0;
        for i in 0..n_keys {
            let key_type = KEY_PATTERN[(start_key + i) % 12];
            key_types[i] = key_type;
            if key_type {
                key_type_indices[i] = n_black_keys;
                n_black_keys += 1;
            } else {
                key_type_indices[i] = n_white_keys;
                n_white_keys += 1;
            }
        }
        Self {
            width,
            height,
            start_key,
            n_keys,
            key_types,
            key_type_indices,
            n_white_keys,
            key_ib_rects: vec![None; n_keys],
        }
    }
}

impl Piano for Piano2D {
    fn get_ib_rect(&mut self, key: usize) -> IbRectangle {
        match &self.key_ib_rects[key] {
            Some(ib_rect) => ib_rect.clone(),
            None => {
                let ib_rect: IbRectangle;
                let white_key_width = self.width / self.n_white_keys as f64;
                if self.key_types[key] {
                    let mut w_group = vec![key - 1, key + 1];
                    let mut b_group = vec![key];
                    let mut i;
                    if key >= 2 {
                        i = key - 2;
                        while self.key_types[i] {
                            b_group.insert(0, i);
                            if i >= 1 {
                                w_group.insert(0, i - 1);
                            }
                            if i < 2 {
                                break;
                            }
                            i -= 2;
                        }
                    }
                    i = key + 2;
                    while i < self.n_keys && self.key_types[i] {
                        b_group.push(i);
                        w_group.push(i + 1);
                        i += 2;
                    }
                    let group_width = white_key_width * (w_group.len() as f64);
                    let group_lx = white_key_width * self.key_type_indices[w_group[0]] as f64;
                    let key_lx = group_lx
                        + (group_width / (2.0 * b_group.len() as f64 + 1.0))
                            * (2.0 * b_group.iter().position(|&k| k == key).unwrap() as f64 + 1.0);
                    let key_rx = key_lx + white_key_width * 0.5;
                    let key_height = self.height * 0.65;
                    ib_rect = IbRectangle {
                        tl: Point2d::new(key_lx, 0.0),
                        tr: Point2d::new(key_rx, 0.0),
                        br: Point2d::new(key_rx, key_height),
                        bl: Point2d::new(key_lx, key_height),
                    };
                } else {
                    let key_lx = self.key_type_indices[key] as f64 * white_key_width;
                    let key_rx = key_lx + white_key_width;
                    ib_rect = IbRectangle {
                        tl: Point2d::new(key_lx, 0.0),
                        tr: Point2d::new(key_rx, 0.0),
                        br: Point2d::new(key_rx, self.height),
                        bl: Point2d::new(key_lx, self.height),
                    };
                }
                self.key_ib_rects[key] = Some(ib_rect);
                ib_rect
            }
        }
    }

    fn get_vector(&mut self, key: usize) -> Vector<Point2d> {
        self.get_ib_rect(key).ib_into()
    }

    fn is_black(&self, key: usize) -> bool {
        self.key_types[key]
    }
}

impl Piano3D {
    pub fn new(border: IbRectangle, piano2d: Piano2D) -> Self {
        let border2d = IbRectangle::new(
            (0.0, 0.0),
            (piano2d.width, 0.0),
            (piano2d.width, piano2d.height),
            (0.0, piano2d.height),
        );
        let src_points: Vector<Point2d> = border2d.ib_into();
        let dst_points: Vector<Point2d> = border.ib_into();
        let h_matrix =
            opencv::calib3d::find_homography(&src_points, &dst_points, &mut no_array(), 0, 5.0)
                .unwrap();
        let n_keys = piano2d.n_keys;
        Piano3D {
            border,
            piano2d,
            h_matrix,
            key_ib_rects: vec![None; n_keys],
        }
    }

    pub fn perspective_transform(&self, src: &Vector<Point2d>) -> Vector<Point2d> {
        let mut dst: Vector<Point2d> = Vector::new();
        opencv::core::perspective_transform(&src, &mut dst, &self.h_matrix).unwrap();
        dst
    }
}

impl Piano for Piano3D {
    fn get_ib_rect(&mut self, key: usize) -> IbRectangle {
        match &self.key_ib_rects[key] {
            Some(ib_rect) => ib_rect.clone(),
            None => {
                let src: Vector<Point2d> = self.piano2d.get_vector(key);
                let ib_rect: IbRectangle = self.perspective_transform(&src).ib_into();
                self.key_ib_rects[key] = Some(ib_rect);
                ib_rect
            }
        }
    }

    fn get_vector(&mut self, key: usize) -> Vector<Point2d> {
        self.get_ib_rect(key).ib_into()
    }

    fn is_black(&self, key: usize) -> bool {
        self.piano2d.key_types[key]
    }
}
