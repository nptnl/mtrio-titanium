#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Val(f32),
    Op(Mfn),
    Begin(i32),
    End(i32),
}
impl Token {
    fn extract(self) -> Result<f32, ()> {
        match self {
            Self::Val(v) => Ok(v),
            _ => Err(println!("you real ugly bruv")),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Mfn {
    Add,
    Sub,
    Mul,
    Div,
    Square,
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read");
    let input = input
    .replace('(', " ( ")
    .replace(')', " ) ");
    let input: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}", complete(tokenize(input)).unwrap())
}
fn tokenize(input: Vec<&str>) -> Vec<Token> {
    let mut tkvec: Vec<Token> = Vec::new();
    let mut depth: i32 = 0;
    for word in input {
        let to_add: Result<Token, ()> = match word {
            "(" => { depth += 1; Ok(Token::Begin(depth)) },
            ")" => { depth -= 1; Ok(Token::End(depth + 1)) },
            "+" | "add" => Ok(Token::Op(Mfn::Add)),
            "-" | "sub" => Ok(Token::Op(Mfn::Sub)),
            "*" | "mul" => Ok(Token::Op(Mfn::Mul)),
            "/" | "div" => Ok(Token::Op(Mfn::Div)),
            "sq" => Ok(Token::Op(Mfn::Square)),
            _ => match word.parse::<f32>() {
                Ok(v) => Ok(Token::Val(v)),
                Err(_) => Err(println!("get good, invalid token")),
            }
        };
        tkvec.push(to_add.unwrap());
    }
    tkvec
}
fn oneop(expr: Vec<Token>) -> Result<f32, ()> {
    let mut output: f32 = 0.0;
    match expr[0] {
        Token::Op(Mfn::Add) => for addend in expr[1..].to_vec() {
            output += addend.extract().unwrap();
        },
        Token::Op(Mfn::Sub) => {
            output = expr[1].extract().unwrap();
            for addend in expr[2..].to_vec() {
                output -= addend.extract().unwrap();
            };
        },
        Token::Op(Mfn::Mul) => {
            output = 1f32;
            for factor in expr[1..].to_vec() {
            output *= factor.extract().unwrap();
            };
        },
        Token::Op(Mfn::Div) => {
            output = expr[1].extract().unwrap();
            for factor in expr[2..].to_vec() {
                output /= factor.extract().unwrap();
            };
        },
        Token::Op(Mfn::Square) => {
            output = 1f32;
            for factor in expr[1..].to_vec() {
                output *= factor.extract().unwrap() * factor.extract().unwrap()
            };
        },
        _ => return Err(println!("invalid operator, returning zero in this operation")),
    };
    Ok(output)
}
fn complete(mut keys: Vec<Token>) -> Result<f32, ()> {
    let le: usize = keys.len();
    match (keys[0], keys[le-1]) {
        (Token::Begin(d1), Token::End(d2)) => if d1 == d2 { return complete(keys[1..le-1].to_vec()) }
        _ => (),
    }
    let mut le: bool = true;
    for word in &keys {
        match word {
            Token::Begin(_d) => { le = false; break },
            _ => continue,
        }
    }
    if le {
        return oneop(keys)
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
        if indx == keys.len() { return begin }
        else if keys[indx] == Token::End(depth) { return indx }
    }
}