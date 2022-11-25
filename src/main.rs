#[derive(Debug, PartialEq, Clone, Copy)]
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
#[derive(Debug, PartialEq, Clone, Copy)]
enum Oprtr {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    let mut input: String = String::new();
    loop {
        std::io::stdin().read_line(&mut input).expect("failed to read");
        let input = input
        .replace('(', " ( ")
        .replace(')', " ) ");
        let input: Vec<&str> = input.split_whitespace().collect();
        println!("{:?}", complete(tokenize(input)).unwrap())
    }
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
fn basic(operation: Token, v1: Token, v2: Token) -> Result<i32, ()> {
    let v1 = v1.extract().unwrap();
    let v2 = v2.extract().unwrap();
    match operation {
        Token::Op(Oprtr::Add) => Ok(v1 + v2),
        Token::Op(Oprtr::Sub) => Ok(v1 - v2),
        Token::Op(Oprtr::Mul) => Ok(v1 * v2),
        Token::Op(Oprtr::Div) => Ok(v1 / v2),
        _ => Err(println!("invalid operator error")),
    }
}
fn complete(mut keys: Vec<Token>) -> Result<i32, ()> {
    if keys.len() == 1 {
        keys[0].extract()
    } else if keys.len() == 3 {
        basic(keys[1], keys[0], keys[2])
    } else if keys.len() == 5 {
        basic(keys[2], keys[1], keys[3])
    } else {
        let mut simpler: Vec<Token> = Vec::new();
        let mut indx: usize = 0;
        while indx < keys.len() {
            match keys[indx] {
                Token::Val(_) => simpler.push(keys[indx]),
                Token::Op(_) => simpler.push(keys[indx]),
                Token::Begin(depth) => {
                    let end = find_end(&keys, indx, depth);
                    let collapsed = complete(keys[indx+1..end].to_vec());
                    keys.drain(indx..end+1);
                    keys.insert(indx, Token::Val(collapsed.unwrap()));
                },
                Token::End(_) => return Err(println!("unopened close-paren"))
            };
            indx += 1;
        }
        complete(keys)
    }
}
fn find_end(keys: &Vec<Token>, begin: usize, depth: i32) -> usize {
    let mut indx = begin;
    loop {
        indx += 1;
        if keys[indx] == Token::End(depth) {
            break
        }
    }
    indx
}