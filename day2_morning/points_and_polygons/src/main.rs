// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::slice::Iter;
use std::ops::Add;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }

    pub fn magnitude(&self) -> f64 {
        return ((self.x * self.x + self.y * self.y) as f64).sqrt();
    }

    pub fn dist(&self, other: &Point) -> f64 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        return ((delta_x * delta_x + delta_y * delta_y) as f64).sqrt();
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        return Point{x: self.x + other.x, y: self.y + other.y};
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    pub fn new() -> Self {
        Polygon{points: vec![]}
    }

    pub fn add_point(&mut self, point: &Point) {
        self.points.push(point.clone());
    }

    pub fn left_most_point(&self) -> Option<&Point> {
        let mut return_point: Option<&Point> = None;
        for point in &self.points {
            if return_point.is_none() || point.x < return_point.unwrap().x {
                return_point = Some(point);
            }
        }
        return return_point;
    }

    pub fn iter(&self) -> Iter<'_, Point> {
        return self.points.iter();
    }

    pub fn circumference(&self) -> f64 {
        let mut val: f64 = 0.0;
        let mut prev_point: Option<&Point> = None;

        for point in &self.points {
            if prev_point.is_some() {
                val += prev_point.unwrap().dist(point);
            }
            prev_point = Some(point);
        }

        // Complete the perimeter.
        if prev_point.is_some() {
            val += prev_point.unwrap().dist(&self.points[0]);
        }

        return val;
    }
}

pub struct Circle {
    point: Point,
    radius: u32,
}

impl Circle {
    pub fn new(point: Point, radius: u32) -> Self {
        Circle{point, radius}
    }

    pub fn circumference(&self) -> f64 {
        return 2.0 * std::f64::consts::PI * (self.radius as f64);
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    pub fn circumference(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.circumference(),
            Shape::Polygon(p) => p.circumference(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(polygon: Polygon) -> Self {
        Shape::Polygon(polygon)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(&p1);
        poly.add_point(&p2);
        assert_eq!(poly.left_most_point(), Some(&p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(&p1);
        poly.add_point(&p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(&Point::new(12, 13));
        poly.add_point(&Point::new(17, 11));
        poly.add_point(&Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
