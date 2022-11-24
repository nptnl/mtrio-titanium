#[derive(Debug, PartialEq)]
enum Token {
    Val(i32),
    Op(Oprtr),
    Begin(i32),
    End(i32),
}
impl Token {
    fn extract(self) -> Result<i32, ()> {
        match self {
            Self::Val(v) => Ok(v),
            _ => Err(println!("you real ugly bruv")),
        }
    }
}
#[derive(Debug, PartialEq)]
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
    println!("{:?}", basic((Token::Op(Oprtr::Add), Token::Val(3), Token::Val(4))))
}
fn tokenize(input: Vec<&str>) -> Vec<Token> {
    let mut tkvec: Vec<Token> = Vec::new();
    let mut depth: i32 = 0;
    for word in input {
        let to_add: Result<Token, ()> = match word {
            "(" => { depth += 1; Ok(Token::Begin(depth)) },
            ")" => { depth -= 1; Ok(Token::End(depth + 1)) },
            "+" => Ok(Token::Op(Oprtr::Add)),
            "-" => Ok(Token::Op(Oprtr::Sub)),
            "*" => Ok(Token::Op(Oprtr::Mul)),
            "//" => Ok(Token::Op(Oprtr::Div)),
            _ => match word.parse::<i32>() {
                Ok(v) => Ok(Token::Val(v)),
                Err(_) => Err(println!("get good, invalid token")),
            }
        };
        tkvec.push(to_add.unwrap());
    }
    tkvec
}
fn basic(set: (Token, Token, Token)) -> Result<i32, ()> {
    let v1 = set.1.extract().unwrap();
    let v2 = set.2.extract().unwrap();
    match set.0 {
        Token::Op(Oprtr::Add) => Ok(v1 + v2),
        Token::Op(Oprtr::Sub) => Ok(v1 - v2),
        Token::Op(Oprtr::Mul) => Ok(v1 * v2),
        Token::Op(Oprtr::Div) => Ok(v1 / v2),
        _ => Err(println!("you suck")),
    }
}