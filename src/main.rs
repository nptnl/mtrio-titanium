#[derive(Debug)]
enum Token {
    Val(i32),
    Op(Oprtr),
    Begin(i32),
    End(i32),
}
#[derive(Debug)]
enum Oprtr {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read");
    let input = input
    .replace('(', " ( ")
    .replace(')', " ) ");
    let input: Vec<&str> = input.split_whitespace().collect();
    println!("{:#?}", tokenize(input))
}

fn tokenize(input: Vec<&str>) {
    let mut tkvec: Vec<Token> = Vec::new();
    let mut depth: i32 = 0;
    for word in input {
        let to_add: Result<Token, ()> = match word {
            "(" => Ok(Token::Begin(depth + 1)),
            ")" => Ok(Token::End(depth - 1)),
            "+" => Ok(Token::Op(Oprtr::Add)),
            "-" => Ok(Token::Op(Oprtr::Sub)),
            "*" => Ok(Token::Op(Oprtr::Mul)),
            "//" => Ok(Token::Op(Oprtr::Div)),
            _ => match word.parse() {
                Ok(v) => Ok(Token::Val(v)),
                Err(_) => Err(println!("get good, invalid token")),
            }
        };
        tkvec.push(to_add.unwrap());
    }
}

/*
i just had an idea i cant forget
so right now im using a Vec<&str> and then tokenizing each word of it
and then later using depth and recursion to do parenthesis
but i can do this while tokenizing, i can calculate the depth
and make the depth an argument to the token enum
like Token::Begin(i32) and Token::End(i32) taking depth into account

*/