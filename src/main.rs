use genawaiter::{stack::let_gen, yield_};
use logos::Logos;
use std::io::{stdin, BufRead};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex(r"[^ \t\r\n\fZ]+", |lex| lex.slice().to_string())]
    Word(String),
}

pub fn do_something_with_tokens(tokens: &mut dyn Iterator<Item = Token>) -> () {
    for token in tokens {
        println!("{:?}", token);
    }
}

fn main() {
    let br = stdin().lock(); // impl BufRead
    let mut lineno = 0;

    let_gen!(token_stream, {
        for readline in br.lines() {
            lineno += 1;
            match readline {
                Ok(line) => {
                    let mut lex = Token::lexer(line.as_str());
                    while let Some(result) = lex.next() {
                        match result {
                            Ok(token) => yield_!(token),
                            Err(_) => {
                                eprintln!(
                                    "unknown token at line {} span {:?}: '{}'",
                                    lineno,
                                    lex.span(),
                                    lex.slice()
                                );
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("IO error! {}", e.to_string());
                    continue;
                }
            }
        }
    });

    let mut tokens = token_stream.into_iter();
    do_something_with_tokens(&mut tokens);
}
