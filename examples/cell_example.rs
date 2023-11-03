use std::cell::Cell;

struct Point {
    x: Cell<i32>,
    y: Cell<i32>,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x: Cell::new(x), y: Cell::new(y) }
    }

    fn move_point(&self, dx: i32, dy: i32) {
        self.x.set(self.x.get() + dx);
        self.y.set(self.y.get() + dy);
    }

    fn get_coordinates(&self) -> (i32, i32) {
        (self.x.get(), self.y.get())
    }
}

fn main() {
    let point = Point::new(2, 3);
    println!("Initial coordinates: {:?}", point.get_coordinates());

    point.move_point(5, -2);
    println!("Updated coordinates: {:?}", point.get_coordinates());
}
