struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        let x_squared = (other.x - self.x).pow(2);
        let y_squared = (other.y - self.y).pow(2);

        (x_squared + y_squared).sqrt()
    }
}

fn main() {
    // Rust has a feature called automatic referencing and dereferencing.
    // Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). T

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 10, y: 5 };

    // Following are the same
    p1.distance(&p2);
    (&p1).distance(&p2);
}
