use opencv::core::{Point, Point2d, VecN, Vector};

use crate::piano::IbRectangle;

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

impl IbFrom<Point2d> for Point {
    fn ib_from(value: Point2d) -> Self {
        Point::new(value.x as i32, value.y as i32)
    }
}

impl IbFrom<Point> for Point2d {
    fn ib_from(value: Point) -> Self {
        Point2d::new(value.x as f64, value.y as f64)
    }
}

impl IbFrom<Vector<Point2d>> for Vector<Point> {
    fn ib_from(value: Vector<Point2d>) -> Self {
        let mut res: Vector<Point> = Vector::new();
        for point in value {
            res.push(point.ib_into());
        }
        res
    }
}

impl IbFrom<Vector<Point>> for Vector<Point2d> {
    fn ib_from(value: Vector<Point>) -> Self {
        let mut res: Vector<Point2d> = Vector::new();
        for point in value {
            res.push(point.ib_into());
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

impl IbFrom<IbRectangle> for Vector<Point2d> {
    fn ib_from(value: IbRectangle) -> Self {
        let mut res: Vector<Point2d> = Vector::new();
        res.push(value.tl);
        res.push(value.tr);
        res.push(value.br);
        res.push(value.bl);
        res
    }
}

impl IbFrom<Vector<Point2d>> for IbRectangle {
    fn ib_from(value: Vector<Point2d>) -> Self {
        IbRectangle {
            tl: value.get(0).unwrap(),
            tr: value.get(1).unwrap(),
            br: value.get(2).unwrap(),
            bl: value.get(3).unwrap(),
        }
    }
}
