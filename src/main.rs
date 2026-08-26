use std::env;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
#[derive(Debug, PartialEq, Clone)]
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
    last: Token,
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
        self.last = if ch.is_alphanumeric() {
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
        };
        return self.last.clone();
    }
}
fn put_file_tokens(file_path: String) {
    let contents = fs::read_to_string(file_path)
        .expect("File does not exist");
    let mut lexer: Lexer = Lexer{
        buf: contents.chars().collect(),
        ptr: 0,
        last: Token::FileEnd,
    };
    let mut tok: Token = lexer.next_token();
    while tok != Token::FileEnd {
        println!("{:?}", tok);
        tok = lexer.next_token();
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
    End,
}
fn parser(lexer: &mut Lexer) -> ASTNode {
    let tok: Token = lexer.next_token();
    match tok {
        Token::OpeningParen => {
            let tok: Token = lexer.next_token();
            if let Token::Operator(Operator) = tok {
                let mut branchs: Vec<ASTNode> = Vec::new();
                while lexer.last != Token::ClosingParen {
                    let new_node: ASTNode = parser(lexer);
                    if new_node == ASTNode::End {
                        break;
                    }
                    else {
                        branchs.push(new_node);
                    }
                }
                ASTNode::Operator(Operator, branchs)
            }
            else {
                panic!("unexpected token {:?}", tok);
            }
        },
        Token::ClosingParen => ASTNode::End,
        Token::Num(i) => ASTNode::Litteral(Value::Int(i)),
        Token::Str(i) => ASTNode::Litteral(Value::Str(i)),
        _ => panic!("unexpected token {:?}", tok),
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: String = args[1].clone();
    // put_file_tokens(file_path);
    let contents = fs::read_to_string(file_path)
        .expect("File does not exist");
    let mut lexer: Lexer = Lexer{
        buf: contents.chars().collect(),
        ptr: 0,
        last: Token::FileEnd,
    };
    let astnode: ASTNode = parser(&mut lexer);
    println!("{:?}", astnode);
}
