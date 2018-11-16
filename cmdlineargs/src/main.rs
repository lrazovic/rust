use std::io::Write;

fn main() {
    if std::env::args().len() < 2 {
        usage();
    }
    print!("Hello");
    for arg in std::env::args().skip(1) {
        print!(" {}", arg);
    }
    println!();
    std::process::exit(0);
}

fn usage() {
    writeln!(std::io::stderr(), "Usage: cargo run 'Your Name'");
    std::process::exit(1);
}
