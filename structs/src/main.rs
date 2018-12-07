struct User {
    name: String,
    surname: String,
    active: bool,
}

fn build_user(name: String, surname: String) -> User {
    User {
        name,
        surname,
        active : true,
    }
}

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn perimeter(&self) -> u32 {
        (self.height + self.width)*2
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

/* Old School, more code way
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
*/

fn main() {
    let name = String::from("Leonardo");
    let surname = String::from("Razovic");
    let mut user = build_user(name, surname);
    user.active = false;
    let figure = Rectangle {height: 30, width: 20};
    let square = Rectangle::square(20);
    println!("Area = {}", figure.area());
    println!("Perimeter = {}", figure.perimeter());
    println!("Area = {}", square.area());
}
