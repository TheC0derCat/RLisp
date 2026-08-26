use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
#[derive(Debug, PartialEq)]
enum Token {
    OpeningParen,
    ClosingParen,
    Identifier(String),
    Str(String),
    Num(i32),
    Operator(Operator),
    FileEnd,
}
struct Lexer {
    buf: Vec<char>,
    ptr: usize,
}
impl Lexer {
    fn getch(&mut self) -> char {
        let ch: char = self.buf[self.ptr];
        // let ch: char = self.buf.as_bytes()[self.ptr] as char;
        self.ptr += 1;
        return ch;
    }
    fn backch(&mut self){
        self.ptr -= 1;
    }
    fn next_token(&mut self) -> Token {
        if self.ptr >= self.buf.len() {
            return Token::FileEnd;
        }
        let mut ch: char = self.getch();
        if ch.is_alphanumeric() {
            let mut tempbuf: String = String::new();
            while ch.is_alphanumeric() {
                tempbuf.push(ch);
                ch = self.getch();
            }
            self.backch();
            match tempbuf.parse::<i32>() {
                Ok(n) => Token::Num(n),
                Err(_) => Token::Identifier(tempbuf),
            }
        }
        else {
            match ch {
                '(' => Token::OpeningParen,
                ')' => Token::ClosingParen,
                '+' => Token::Operator(Operator::Add),
                '-' => Token::Operator(Operator::Sub),
                '*' => Token::Operator(Operator::Mul),
                '/' => Token::Operator(Operator::Div),
                '%' => Token::Operator(Operator::Mod),
                '"' => {
                    let mut tempbuf: String = String::new();
                    ch = self.getch();
                    while ch != '"' {
                        tempbuf.push(ch);
                        ch = self.getch();
                    }
                    Token::Str(tempbuf)
                },
                _ => self.next_token(),
            }
        }
    }
}
#[derive(Debug, PartialEq)]
enum Value {
    Int(i32),
    Str(String),
}
#[derive(Debug, PartialEq)]
enum ASTNode {
    Litteral(Value),
    Operator(Operator, Vec<ASTNode>),
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: String = args[1].clone();
    let contents = fs::read_to_string(file_path)
        .expect("File does not exist");
    let mut lexer: Lexer = Lexer{
        buf: contents.chars().collect(),
        ptr: 0,
    };
    let mut tok: Token = lexer.next_token();
    while tok != Token::FileEnd {
        println!("{:?}", tok);
        tok = lexer.next_token();
    }
}
