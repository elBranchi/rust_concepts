use std::{
    fmt,
    ops::{Add, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point2D {
    x: i16,
    y: i16,
}
#[derive(Debug, Copy, Clone, Default)]
struct Vector2D {
    x: i16,
    y: i16,
}
enum Line2D {
    WithDirection(Point2D, Vector2D),
    WithEndPoint(Point2D, Point2D),
    FromOrigin(Vector2D),
}

#[derive(Debug)]
struct Line2DIterator {
    dx: i16,
    dy: i16,
    unit_x: i16,
    unit_y: i16,
    div_x: i16,
    div_y: i16,
    ac_x: i16,
    ac_y: i16,
    next: Option<Point2D>,
    end: Point2D,
}

impl Line2D {
    fn iter(&self) -> Line2DIterator {
        self.into_iter()
    }
}

impl Line2DIterator {
    fn new(dx: i16, dy: i16, origin: Point2D, end: Point2D) -> Line2DIterator {
        let unit_x = if dx >= 0 { 1 } else { -1 };
        let unit_y = if dy >= 0 { 1 } else { -1 };
        let max_d = (if dx.abs() > dy.abs() { dx } else { dy }).abs();
        Line2DIterator {
            dx,
            dy,
            div_x: max_d * unit_x,
            div_y: max_d * unit_y,
            unit_x,
            unit_y,
            ac_x: 0,
            ac_y: 0,
            next: Some(origin),
            end,
        }
    }
}

impl Iterator for Line2DIterator {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;

        if self.next != Option::None {
            let mut next = current?;
            self.ac_x += self.dx;
            self.ac_y += self.dy;
            // print!("[{}, {}]", self.ac_x, self.ac_y);
            if self.ac_x / self.div_x != 0 {
                self.ac_x %= self.div_x;
                next.x += self.unit_x;
            }

            if self.ac_y / self.div_y != 0 {
                self.ac_y %= self.div_y;
                next.y += self.unit_y;
            }
            // println!("=> [{}, {}]", self.ac_x, self.ac_y);
            self.next = if next != self.end {
                Some(next)
            } else {
                Option::None
            };
        }
        current
    }
}
impl IntoIterator for &Line2D {
    type Item = Point2D;

    type IntoIter = Line2DIterator;

    fn into_iter(self) -> Self::IntoIter {
        let iter = match self {
            Line2D::FromOrigin(direction) => {
                let origin = Point2D::default();
                Line2DIterator::new(direction.x, direction.y, origin, origin + *direction)
            }
            Line2D::WithEndPoint(origin, end) => {
                let direction = *end - *origin;
                Line2DIterator::new(direction.x, direction.y, *origin, *end)
            }
            Line2D::WithDirection(origin, direction) => {
                let end = *origin + *direction;
                Line2DIterator::new(direction.x, direction.y, *origin, end)
            }
        };
        iter
    }
}

impl Default for Point2D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(i16, i16)> for Point2D {
    fn from(value: (i16, i16)) -> Self {
        Point2D {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<(i16, i16)> for Vector2D {
    fn from(value: (i16, i16)) -> Self {
        Vector2D {
            x: value.0,
            y: value.1,
        }
    }
}
impl Add<Vector2D> for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub<Point2D> for Point2D {
    type Output = Vector2D;
    fn sub(self, rhs: Point2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
fn main() {
    let p = Point2D::default();
    let p1 = Point2D { x: 16, y: -11 };
    println!("{p}");
    //As Point2D has Copy trait, assignment to p2 does not invalidate p
    let p2 = p;
    let p3: Point2D = (-2i16, 3i16).into();
    println!("{p}");

    let v = p1 - p2;
    let v2 = p1 - p3;
    println!("vect {v:?}, {v2:?}");

    let line = Line2D::FromOrigin(v);

    let line_iter = line.iter();
    println!("{line_iter:?}");
    for pair in line_iter.enumerate() {
        let (i, p) = pair;
        println!("[{i}] = {p}");
    }

    let l_iter = Line2D::WithEndPoint(p1, p3).into_iter();
    println!("{l_iter:?}");
    for point in l_iter.enumerate(){

        println!("{point:?}");
    }
}
