use std::fs;
use std::collections::HashMap;
use std::env;

mod lexer;
use lexer::*;

mod parser;
use parser::*;

mod walker;
use walker::*;

fn interpret(contents: String) -> Value {
    let mut lexer: Lexer = Lexer{
        buf: contents.chars().collect(),
        ptr: 0,
        last: Token::FileEnd,
    };
    let mut program_state: ProgramState = ProgramState{
        variables: HashMap::new(),
    };
    let astnode: ASTNode = parser(&mut lexer);
    let value: Value = walker(&astnode, &mut program_state);
    return value;
}
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_math() {
        assert_eq!(interpret("( + 1 2)".to_string()), Value::Int(3));
        assert_eq!(interpret("(- 5 1) ".to_string()), Value::Int(4));
        assert_eq!(interpret(" (* 3 5)".to_string()), Value::Int(15));
        assert_eq!(interpret("(/ 20 2 )".to_string()), Value::Int(10));
    }

    #[test]
    fn test_equality() {
        assert_eq!(interpret(" (== 1 2)".to_string()), Value::Bool(false));
        assert_eq!(interpret("( == 5 5)".to_string()), Value::Bool(true));
    }

    #[test]
    fn test_or() {
        assert_eq!(interpret("(| true false)".to_string()), Value::Bool(true));
        assert_eq!(interpret("(| false false)".to_string()), Value::Bool(false));
        assert_eq!(interpret("(| true true )".to_string()), Value::Bool(true));
    }
    #[test]
    fn test_and() {
        assert_eq!(interpret(" (& true false)".to_string()), Value::Bool(false));
        assert_eq!(interpret("( & false false)".to_string()), Value::Bool(false));
        assert_eq!(interpret("(& true true )".to_string()), Value::Bool(true));
    }
}
