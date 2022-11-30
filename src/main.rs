use ferrum::ch::Comp;
use ferrum::alg::{exp, ln};
use std::collections::{HashMap};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Val(Comp),
    Op(Mfn),
    Begin(i32),
    End(i32),
}
impl Token {
    fn extract(self) -> Result<Comp, ()> {
        match self {
            Self::Val(v) => Ok(v),
            _ => Err(println!("you real ugly bruv")),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Mfn {
    Add, Sub, Mul, Div,
    Square,
    Exp, Pow, Ln, Log,
}

fn main() {
    let mut variables: HashMap<String, Comp> = HashMap::new();
    let mut ans: Comp = ferrum::ch::CC0;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read");
        if &input[..2] == "->" {
            variables.insert(
                input.trim()[3..].to_string(),
                ans,
            );
            println!("{:?}", variables[&input.trim()[3..].to_string()]);
            continue;
        } else if &input.trim() == &"var" {
            for (name, value) in variables.clone() {
                println!("{name} = {value:?}");
            }
        } else {
            input = input.replace('(', " ( ").replace(')', " ) ");
            let input: Vec<&str> = input.split_whitespace().collect();
            ans = complete(tokenize(input)).unwrap();
            println!("{ans:?}");
        }
    }
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
            "exp" => Ok(Token::Op(Mfn::Exp)),
            "pow" | "^" | "**" => Ok(Token::Op(Mfn::Pow)),
            "ln" => Ok(Token::Op(Mfn::Ln)),
            "log" => Ok(Token::Op(Mfn::Log)),
            _ => match word.parse::<Comp>() {
                Ok(v) => Ok(Token::Val(v)),
                Err(_) => Err(println!("get good, invalid token")),
            }
        };
        tkvec.push(to_add.unwrap());
    }
    tkvec
}
fn oneop(expr: Vec<Token>) -> Result<Comp, ()> {
    let mut output: Comp = ferrum::ch::CC0;
    match expr[0] {
        Token::Val(v) => output = v,
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
            output = ferrum::ch::CC1;
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
            output = ferrum::ch::CC1;
            for factor in expr[1..].to_vec() {
                output *= factor.extract().unwrap() * factor.extract().unwrap()
            };
        },
        Token::Op(Mfn::Exp) => {
            for addend in expr[1..].to_vec() {
                output += addend.extract().unwrap();
            }
            output = exp(output);
        },
        Token::Op(Mfn::Pow) => {
            for addend in expr[2..].to_vec() {
                output += addend.extract().unwrap();
            }
            output = expr[1].extract().unwrap().pow(output);
        },
        Token::Op(Mfn::Ln) => {
            output = ferrum::ch::CC1;
            for factor in expr[1..].to_vec() {
                output *= factor.extract().unwrap();
            }
            output = ln(output);
        },
        Token::Op(Mfn::Log) => {
            output = ferrum::ch::CC1;
            for factor in expr[2..].to_vec() {
                output *= factor.extract().unwrap();
            }
            output = output.log(expr[1].extract().unwrap());
        }
        _ => return Err(println!("invalid operator, returning zero in this operation")),
    };
    Ok(output)
}
fn complete(mut keys: Vec<Token>) -> Result<Comp, ()> {
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