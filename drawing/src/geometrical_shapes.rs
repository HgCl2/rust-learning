use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&mut self, canva: &mut Image);
    fn color() -> Color;
}

pub trait Displayable {
    fn display(canva: &mut Image, x: i32, y: i32, color: Color);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x_coord: i32, y_coord: i32) -> Self {
        Self {
            x: x_coord,
            y: y_coord,
        }
    }

    pub fn clone(&self) -> Point {
        return Point {
            x: self.x,
            y: self.y,
        };
    }

    pub fn random(width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        return Point { x: rng.gen_range(0, width+1), 
            y: rng.gen_range(0, height)};
    }

}

impl Drawable for Point{
    fn color() -> Color {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(1, 255);
        let g = rng.gen_range(1, 255);
        let b = rng.gen_range(1, 255);

        return Color::rgb(r, g, b);
    }

    fn draw(&mut self, canva: &mut Image){
        Image::display(canva, self.x, self.y, Point::color())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    pub start_point: Point,
    pub finish_point: Point,
}

impl Drawable for Line {
    fn draw(&mut self, canva: &mut Image) {
        let color = Line::color();

        let mut cur_point = self.start_point.clone();
        let finish_point = &self.finish_point;

        let dx = (finish_point.x - cur_point.x).abs();
        let dy = (finish_point.y - cur_point.y).abs();

        let mut pk = 2 * dy - dx;

        for _i in 0..(dx + 1) {
            Image::display(canva, cur_point.x, cur_point.y, 
                Color::rgb(self.r, self.g, self.b));

            if cur_point.x < finish_point.x {
                cur_point.x += 1;
            } else {
                cur_point.x -= 1;
            }

            if pk < 0 {
                if dx > dy {
                    pk = pk + 2 * dy;
                } else {
                    pk = pk + 2 * dx;
                }
            } else {
                if cur_point.y < finish_point.y {
                    cur_point.y += 1;
                } else {
                    cur_point.y -= 1;
                }
                pk = pk + 2 * dy - 2 * dx;
            }
        }
    }

    fn color() -> Color {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(1, 255);
        let g = rng.gen_range(1, 255);
        let b = rng.gen_range(1, 255);

        return Color::rgb(r, g, b);
    }
}

impl Line {
    pub fn random(width: i32, height: i32) -> Line {
        let mut rng = rand::thread_rng();

        let first_point: Point =
            Point::new(rng.gen_range(1, width + 1), rng.gen_range(1, height + 1));
        let second_point: Point =
            Point::new(rng.gen_range(1, width + 1), rng.gen_range(1, height + 1));

        return Line {
            start_point: first_point,
            finish_point: second_point,
        };
    }
    pub fn new(start_point: Point, finish_point: Point) -> Line {
        return Line { start_point: start_point, 
            finish_point: finish_point};
    }
}

pub struct Triangle {
    pub first_point: Point,
    pub second_point: Point,
    pub third_point: Point,
}

impl Drawable for Triangle{
    fn draw(&mut self, canva: &mut Image) {
        let mut first_line = Line::new(self.first_point.clone(), self.second_point.clone());
        let mut second_line = Line::new(self.second_point.clone(), self.third_point.clone());
        let mut third_line = Line::new(self.third_point.clone(), self.first_point.clone());


        first_line.draw(canva);
        second_line.draw(canva);
        third_line.draw(canva);
    }

    fn color() -> Color {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(0, 255);
        let g = rng.gen_range(0, 255);
        let b = rng.gen_range(0, 255);

        return Color::rgb(r, g, b);
    }
}

impl Triangle {
    pub fn new(first_p: &Point, second_p: &Point, third_p: &Point) -> Self{
        Self {
            first_point: first_p.clone(),
            second_point: second_p.clone(),
            third_point: third_p.clone(),
        }
    }
}

impl Displayable for Image {
    fn display(canva: &mut Image, x: i32, y: i32, color: Color) {
        if x >= 0 && x < canva.width && y >= 0 && y < canva.height {
            canva.set_pixel(x, y, color).unwrap();
        }
    }
}
