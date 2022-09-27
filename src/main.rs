trait Shape {
    fn area(&self) -> usize {
        self.get_length() * self.get_width()
    }
   
    fn get_length(&self) -> usize;
    fn get_width(&self) -> usize;
   
    fn rand_name(&self) -> &'static str {
        "Hapa kule!"
    }
}

struct Square(usize);

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Square")
    }
}

impl Shape for Square {
    fn get_width(&self) -> usize {
        self.0
    }
   
    fn get_length(&self) -> usize {
        self.0
    }
}

struct Triangle {
    height: usize,
    base: usize
}

impl std::fmt::Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Triangle")
    }
}

impl Shape for Triangle {
    fn area(&self) -> usize {
        (self.get_length() as f32 * self.get_width() as f32 * 0.5) as usize
    }
   
    fn get_width(&self) -> usize {
        self.base
    }
   
    fn get_length(&self) -> usize {
        self.height
    }
}

struct Rectangle {
    length: usize,
    width: usize
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Rectagle")
    }
}

impl Shape for Rectangle {
    fn get_width(&self) -> usize {
        self.width
    }
   
    fn get_length(&self) -> usize {
        self.length
    }
}

fn main_dup<T,U,V>(square: T, rect: U, tri: V)
where
    T: Shape + std::fmt::Display,
    U: Shape + std::fmt::Display,
    V: Shape + std::fmt::Display,
 {
    println!("Areas:\nRect = {}\nSquare = {}\nTriangle = {}", rect.area(), square.area(), tri.area());
    println!("Text:\nRect = {}\nSquare = {}\nTriangle = {}", rect.rand_name(), square.rand_name(), tri.rand_name());
    println!("Areas:\nRect = {}\nSquare = {}\nTriangle = {}", rect, square, tri);
}

fn main() {
    let square = Square(10);
    let rect = Rectangle {
        width: 10,
        length: 20,
    };
    let tri = Triangle {
        base: 10,
        height: 20,
    };
    main_dup(square, rect, tri);
}