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

fn main() {
    let name = String::from("Leonardo");
    let surname = String::from("Razovic");
    let mut user = build_user(name, surname);
    println!("Name: {}", user.name);
    println!("Surname: {}", user.surname);
    println!("Active: {}", user.active);
    println!("Deactivating...");
    user.active = false;
    println!("Active: {}", user.active);
}
