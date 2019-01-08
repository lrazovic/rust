fn main() {
    let mut v: Vec<i32> = Vec::new();
    let z = vec![1,2,3];
    v.push(1);
    println!("First element of Z: {}", z[0]);
    for i in &z {
        println!("z-ith {}", i)
    }
    println!("First element of V: {}", v[0]);
}
