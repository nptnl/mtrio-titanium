use ferrum::ch::Comp;
use ferrum::alg::{exp, ln};
use std::collections::{HashMap};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Val(Comp),
    Op(Mfn),
    Begin(i32),
    End(i32),
    Inp,
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
    let mut functions: HashMap<String, Vec<Token>> = HashMap::new();
    let mut ans: Comp;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read");
        let input = input.replace('(', " ( ").replace(')', " ) ");
        let working: Vec<&str> = input.split_whitespace().collect();

        match working[0] {
            "ass" => {
                variables.insert(
                working[1].to_string(),
                complete(tokenize(working[2..].to_vec(), &variables, &functions)).unwrap(),
                );
            },
            "def" => {
                functions.insert(
                working[1].to_string(),
                tokenize(working[2..].to_vec(), &variables, &functions),
                );
            },
            "variables" => for (name, value) in variables.clone() {
                println!("{name} = {value:?}");
            },
            _ => {
                ans = complete(tokenize(working, &variables, &functions)).unwrap();
                println!("{ans:?}");
            },
        };
    }
}
fn tokenize(input: Vec<&str>, varlist: &HashMap<String, Comp>, fnlist: &HashMap<String, Vec<Token>>) -> Vec<Token> {
    let mut tkvec: Vec<Token> = Vec::new();
    let mut depth: i32 = 0;
    for (indx, word) in input.iter().enumerate() {
        let to_add: Token = match *word {
            "(" => { depth += 1; Token::Begin(depth) },
            ")" => { depth -= 1; Token::End(depth + 1) },
            "o" => Token::Inp,
            "+" | "add" => Token::Op(Mfn::Add),
            "-" | "sub" => Token::Op(Mfn::Sub),
            "*" | "mul" => Token::Op(Mfn::Mul),
            "/" | "div" => Token::Op(Mfn::Div),
            "sq" => Token::Op(Mfn::Square),
            "exp" => Token::Op(Mfn::Exp),
            "pow" | "^" | "**" => Token::Op(Mfn::Pow),
            "ln" => Token::Op(Mfn::Ln),
            "log" => Token::Op(Mfn::Log),
            _ => {
                let varposs: Option<Comp> = varlist.get(*word).copied();
                let fnposs: Option<&Vec<Token>> = fnlist.get(*word);
                let output: Token;
                match varposs {
                    Some(v) => output = Token::Val(v),
                    None => match fnposs {
                        Some(v) => output = Token::Val(evaluate(v.clone(), input[indx+1].parse::<Comp>().unwrap()).unwrap()),
                        None => match word.parse::<Comp>() {
                            Ok(v) => output = Token::Val(v),
                            Err(_) => panic!("invalid token lmao"),
                        }
                    },
                };
                output
            },
        };
        tkvec.push(to_add);
    };
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
fn evaluate(func: Vec<Token>, inp: Comp) -> Result<Comp, ()> {
    let new: Vec<Token> = func.into_iter().map(|x| if x == Token::Inp { Token::Val(inp) } else { x } ).collect();
    complete(new)
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
            Token::Begin(_) => { le = false; break },
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
                Token::End(_) => return Err(println!("unopened close-paren")),
                _ => panic!("invalid token lmao"),
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