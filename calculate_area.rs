enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(rad) => std::f64::consts::PI * rad * rad,
        Shape::Square(side) => side * side,
        Shape::Rectangle(l, b) => l * b,
    }
}

fn main() {

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(5.0);
    let rectangle = Shape::Rectangle(5.0, 6.0);

    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
}

