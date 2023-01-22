use opencv::core::{Point, Point2d, VecN, Vector};

use crate::piano::PianoBorder;

pub trait IbFrom<T> {
    fn ib_from(value: T) -> Self;
}

pub trait IbInto<T> {
    fn ib_into(self) -> T;
}

impl<T, U> IbInto<U> for T
where
    U: IbFrom<T>,
{
    fn ib_into(self) -> U {
        U::ib_from(self)
    }
}

impl IbFrom<Vector<Point2d>> for Vector<Point> {
    fn ib_from(value: Vector<Point2d>) -> Self {
        let mut res: Vector<Point> = Vector::new();
        for point in value {
            res.push(Point::new(point.x as i32, point.y as i32));
        }
        res
    }
}

impl IbFrom<Vector<Point>> for Vector<Point2d> {
    fn ib_from(value: Vector<Point>) -> Self {
        let mut res: Vector<Point2d> = Vector::new();
        for point in value {
            res.push(Point2d::new(point.x as f64, point.y as f64));
        }
        res
    }
}

impl IbFrom<(f64, f64, f64)> for VecN<f64, 4> {
    fn ib_from(value: (f64, f64, f64)) -> Self {
        VecN::new(value.0, value.1, value.2, 0.0)
    }
}

impl IbFrom<(f64, f64)> for Point2d {
    fn ib_from(value: (f64, f64)) -> Self {
        Point2d::new(value.0, value.1)
    }
}

impl IbFrom<PianoBorder> for Vector<Point2d> {
    fn ib_from(value: PianoBorder) -> Self {
        let mut res: Vector<Point2d> = Vector::new();
        res.push(value.tl);
        res.push(value.tr);
        res.push(value.br);
        res.push(value.bl);
        res
    }
}
