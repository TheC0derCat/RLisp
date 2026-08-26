use std::env;
use std::fs;

#[derive(Debug)]
enum Token {
    OpeningParen,
    ClosingParen,
    Identifier(String),
    Str(String),
    Num(i32),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
struct Lexer {
    buf: String,
    ptr: usize,
}
impl Lexer {
    fn len(&self) -> usize {
        return self.buf.as_bytes().len();
    }
    fn getch(&mut self) -> char {
        let ch: char = self.buf.as_bytes()[self.ptr] as char;
        self.ptr += 1;
        return ch;
    }
    fn backch(&mut self){
        self.ptr -= 1;
    }
    fn next_token(&mut self) -> Token {
        let mut ch: char = self.getch();
        if ch.is_alphanumeric() {
            let mut tempbuf: String = String::new();
            while ch.is_alphanumeric() {
                tempbuf.push(ch);
                ch = self.getch();
            }
            match tempbuf.parse::<i32>() {
                Ok(n) => Token::Num(n),
                Err(_) => Token::Identifier(tempbuf),
            }
        }
        else {
            match ch {
                '(' => Token::OpeningParen,
                ')' => Token::ClosingParen,
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Mul,
                '/' => Token::Div,
                '%' => Token::Mod,
                _ => self.next_token(),
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: String = args[1].clone();
    let contents = fs::read_to_string(file_path)
        .expect("File does not exist");
    let mut lexer: Lexer = Lexer{
        buf: contents,
        ptr: 0,
    };
    while lexer.ptr < lexer.len() {
        println!("{:?}", lexer.next_token());
    }
}
