use std::fs;
use std::collections::HashMap;
use std::env;

mod lexer;
use lexer::*;

mod parser;
use parser::*;

mod walker;
use walker::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: String = args[1].clone();
    put_file_tokens(file_path.clone());
    let contents = fs::read_to_string(file_path)
        .expect("File does not exist");
    let mut lexer: Lexer = Lexer{
        buf: contents.chars().collect(),
        ptr: 0,
        last: Token::FileEnd,
    };
    let mut program_state: ProgramState = ProgramState{
        variables: HashMap::new(),
    };
    let astnode: ASTNode = parser(&mut lexer);
    println!("{:?}", astnode);
    let value: Value = walker(&astnode, &mut program_state);
    println!("{:?}", value);
}
