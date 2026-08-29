use crate::lexer::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
    Lambda(Box<ASTNode>),
}
impl Value {
    pub fn extract_int(&self) -> i32 {
        match self {
            Value::Int(i) => i.clone(),
            Value::Str(i) => i
                .parse::<i32>()
                .expect("cant extract int out of non number"),
            Value::Bool(i) => {
                if *i {
                    1
                } else {
                    0
                }
            }
            Value::Lambda(_) => panic!("cant extract int out of non number")
        }
    }
    pub fn extract_bool(&self) -> bool {
        match self {
            Value::Bool(i) => i.clone(),
            Value::Int(i) => match i {
                1 => true,
                0 => false,
                _ => panic!("cant extract bool from non bool"),
            },
            Value::Str(i) => match i.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("cant extract bool from non bool"),
            },
            Value::Lambda(_) => panic!("cant extract bool from non bool")
        }
    }
    pub fn extract_lambda(&self) -> ASTNode {
        match self {
            Value::Lambda(i) => <ASTNode as Clone>::clone(&**i),
            _ => panic!("cant extract bool from non bool")
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode {
    Litteral(Value),
    Operator(Operator, Vec<ASTNode>),
    Identifier(String),
    LambdaCall(String, Box<ASTNode>),
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
                    } else {
                        branchs.push(new_node);
                    }
                }
                ASTNode::Operator(operator, branchs)
            } else {
                if let Token::Identifier(identifier) = tok {
                    let new_node: ASTNode = parser(lexer);
                    ASTNode::LambdaCall(identifier, Box::new(new_node))
                }
                else {
                    panic!("unexpected token {:?}, expected operator instead", tok);
                }
            }
        }
        Token::ClosingParen => ASTNode::End,
        Token::Num(i) => ASTNode::Litteral(Value::Int(i)),
        Token::Str(i) => ASTNode::Litteral(Value::Str(i)),
        Token::True => ASTNode::Litteral(Value::Bool(true)),
        Token::False => ASTNode::Litteral(Value::Bool(false)),
        Token::Identifier(i) => ASTNode::Identifier(i),
        _ => panic!("unexpected token {:?}", tok),
    }
}
