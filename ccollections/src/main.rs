use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let z = vec![1,2,3];
    v.push(1);
    println!("First element of Z: {}", z[0]);
    for i in &z {
        println!("z-ith {}", i)
    }
    println!("First element of V: {}", v[0]);

    // let mut s = String::new();
    let data = "string type inference";
    let data_string = data.to_string();
    println!("dataString: {}", data_string);

    // A String can grow in size and its contents can change

    let mut s = String::from("uno");
    s.push_str(" due");
    println!("s final form: '{}'", s);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + &s2 + &s3;
    let s = format!("{}-{}-{}",s1,s2,s3);
    println!("s: '{}'", s);
    println!("lenght of s: {}", s.len());
    for c in s.chars() {
        println!("{}", c);
    }
    let teams = vec![String::from("Vikings"), String::from("Lions")];
    let initial_scores = vec![0,0];
    let scores: HashMap <_,_> = teams.iter().zip(initial_scores.iter()).collect();
    for (key,value) in &scores {
        println!("{}: {}", key,value);
    }
}
