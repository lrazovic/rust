fn main() {
    let s = String::from("Hello");
    /*
    take(s);
    let x = 5;
    make(x);
    */
    let s1 = s.clone();
    let len = calculate_len(&s1);
    println!("The length of '{}' is [{}]", s1, len);
}

fn take(some: String) {
    println!("{}", some)
}

fn make(integer: i32) {
    println!("{}", integer)
}

fn calculate_len(strpointer: &String) -> usize {
    strpointer.len()
}
