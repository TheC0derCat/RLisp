use crate::lexer::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
}
impl Value {
    pub fn extract_int(&self) -> i32 {
        match self {
            Value::Int(i) => i.clone(),
            Value::Str(i) => i.parse::<i32>().expect("cant extract int out of non number"),
            Value::Bool(i) => if *i {1} else {0},
        }
    }
    pub fn extract_bool(&self) -> bool {
        match self {
            Value::Bool(i) => i.clone(),
            _ => panic!("Cant extract int out of non number"),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Litteral(Value),
    Operator(Operator, Vec<ASTNode>),
    Identifier(String),
    End,
}
impl ASTNode {
    pub fn extract_identifier(&self) -> String {
        match self {
            ASTNode::Identifier(i) => i.clone(),
            _ => panic!("Cant extract identifier out of non identifier"),
        }
    }
}
pub fn parser(lexer: &mut Lexer) -> ASTNode {
    let tok: Token = lexer.next_token();
    match tok {
        Token::OpeningParen => {
            let tok: Token = lexer.next_token();
            if let Token::Operator(operator) = tok {
                let mut branchs: Vec<ASTNode> = Vec::new();
                loop {
                    let new_node: ASTNode = parser(lexer);
                    if new_node == ASTNode::End {
                        break;
                    }
                    else {
                        branchs.push(new_node);
                    }
                }
                ASTNode::Operator(operator, branchs)
            }
            else {
                panic!("unexpected token {:?}, expected operator instead", tok);
            }
        },
        Token::ClosingParen => ASTNode::End,
        Token::Num(i) => ASTNode::Litteral(Value::Int(i)),
        Token::Str(i) => ASTNode::Litteral(Value::Str(i)),
        Token::True => ASTNode::Litteral(Value::Bool(true)),
        Token::False => ASTNode::Litteral(Value::Bool(false)),
        Token::Identifier(i) => ASTNode::Identifier(i),
        _ => panic!("unexpected token {:?}", tok),
    }
}
