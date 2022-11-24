
use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let input = input
    .replace('(', " ( ")
    .replace(')', "");
    let input: Vec<&str> = input
    .split_whitespace()
    .collect();
    println!("yes yes working?? {}", complete(input));
}

fn oprtn(op: &str, lhs: i32, rhs: i32) -> i32 {
    match op {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*'" => lhs * rhs,
        "//" => lhs / rhs,
        _ => 0,
    }
}
fn complete(inp: Vec<&str>) -> i32 {
    let (mut var1, mut var2): (i32, i32) = (0, 0);
    if inp[0] == "(" {
        var1 = match find_end(inp[1..].to_vec()) {
            Ok(v) => complete(inp[1..v].to_vec()),
            Err(()) => return 0,
        }
    }
    
}
fn find_end(remain: Vec<&str>) -> Result<usize, ()> {
    let mut depth: i32 = 1;
    for (indx, ltr) in remain.iter().enumerate() {
        if ltr == &"(" { depth += 1 }
        else if ltr == &")" { depth -= 1 };
        if depth == 0 { return Ok(indx) }
    };
    Err(println!("unclosed paren error"))
}