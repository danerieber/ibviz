use opencv::core::{Point, Point2d, VecN, Vector};

use crate::piano::PianoBorder;

pub trait IbvFrom<T> {
    fn ibv_from(value: T) -> Self;
}

pub trait IbvInto<T> {
    fn ibv_into(self) -> T;
}

impl<T, U> IbvInto<U> for T
where
    U: IbvFrom<T>,
{
    fn ibv_into(self) -> U {
        U::ibv_from(self)
    }
}

impl IbvFrom<Vector<Point2d>> for Vector<Point> {
    fn ibv_from(value: Vector<Point2d>) -> Self {
        let mut res: Vector<Point> = Vector::new();
        for point in value {
            res.push(Point::new(point.x as i32, point.y as i32));
        }
        res
    }
}

impl IbvFrom<Vector<Point>> for Vector<Point2d> {
    fn ibv_from(value: Vector<Point>) -> Self {
        let mut res: Vector<Point2d> = Vector::new();
        for point in value {
            res.push(Point2d::new(point.x as f64, point.y as f64));
        }
        res
    }
}

impl IbvFrom<(f64, f64, f64)> for VecN<f64, 4> {
    fn ibv_from(value: (f64, f64, f64)) -> Self {
        VecN::new(value.0, value.1, value.2, 0.0)
    }
}

impl IbvFrom<(f64, f64)> for Point2d {
    fn ibv_from(value: (f64, f64)) -> Self {
        Point2d::new(value.0, value.1)
    }
}

impl IbvFrom<PianoBorder> for Vector<Point2d> {
    fn ibv_from(value: PianoBorder) -> Self {
        let mut res: Vector<Point2d> = Vector::new();
        res.push(value.tl);
        res.push(value.tr);
        res.push(value.br);
        res.push(value.bl);
        res
    }
}
