use std::io;

fn main() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    println!("{}", threeword(&input));
}

fn threeword(input: &str) -> i32 {
    let wordvec: Vec<&str> = input.split_whitespace().collect();
    let lhs: i32 = wordvec[0].parse().unwrap();
    let rhs: i32 = wordvec[2].parse().unwrap();
    match wordvec[1] {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        _ => 0,
    }
}