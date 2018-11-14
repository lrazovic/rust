fn main() {
    // const AGE: u32 = 22;
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 6;
    println!("The value of x is: {}", x);
    let x = x + 6;
    println!("The value of x is: {}", x);
    let spaces = "   ";
    let _spaces = spaces.len();
    let guess: isize = "73".parse().expect("NaN");
    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    let _a = [1, 2, 3, 4, 5];
    let _index = 10;
    //let _element = a[index];
    another_function(guess, five());
    if five() < guess {
        println!("{} is less than {}", five(), guess);
    } else {
        println!("{} is greater than {}", five(), guess);
    }
    let condition : bool = false;
    let number = if condition {
        5
    } else {
        6
    };
    println!("if condition {}", number);
    let mut k = 0;
    let result = loop {
        k += 1;
        if k == 10 {
            break k*2;
        }
    };

    assert_eq!(result, 20);
    let a = [1,2,3,4,5,6,7];
    for elem in a.iter() {
        println!("{}", elem)
    }
}

fn another_function(x: isize, y: isize) {
    println!("Value of 'guess' is {}, value of 'five()' is {}", x, y)
}

fn five() -> isize {
    5
}
