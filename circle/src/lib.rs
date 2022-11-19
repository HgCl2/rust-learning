#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center_x: f64, center_y: f64, rad: f64) -> Self{
        Self {
            center: Point { x: center_x, y: center_y},
            radius: rad,
        }
    }

    pub fn diameter(&self) -> f64 {
        return 2.0 * self.radius;
    }

    pub fn area(&self) -> f64 {
        return std::f64::consts::PI * self.radius * self.radius;
    }

    pub fn intersect(&self, second_circle: &Circle) -> bool{
        let distance_between_centers = second_circle.center.distance(&self.center);
        return distance_between_centers < self.radius + second_circle.radius;
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, second_point: &Point) -> f64 {
        return ((second_point.x - self.x).powf(2.0) + 
            (second_point.y - self.y).powf(2.0)).sqrt();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance() {
        let point_a = Point { x: 1.0, y: 1.0 };
        let point_b = Point { x: 0.0, y: 0.0 };
        assert_eq!(point_a.distance(&point_b), 1.4142135623730951);
    }

    #[test]
    fn circle() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point { x: 80.0, y: 115.0 },
            radius: 30.0,
        };

        assert_eq!(circle.area(), 70685.83470577035);
        assert_eq!(circle.diameter(), 300.0);
        assert_eq!(circle1.diameter(), 60.0);
        assert_eq!(circle.intersect(&circle1), false);
    }

}
