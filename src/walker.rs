use crate::parser::*;
use crate::lexer::*;
use std::collections::HashMap;

pub struct ProgramState {
    pub variables: HashMap<String, Value>,
}
pub fn domath<F: Fn(i32, i32) -> i32>(branchs: &Vec<ASTNode>, f: F, mut program_state: &mut ProgramState) -> Value {
    let mut value: i32 = walker(&branchs[0], &mut program_state).extract_int();
    let mut i: usize = 1;
    while i < branchs.len() {
        let inti: i32 = walker(&branchs[i], &mut program_state).extract_int();
        value = f(value, inti);
        i += 1;
    }
    Value::Int(value)
}
pub fn dologic<F: Fn(bool, bool) -> bool>(branchs: &Vec<ASTNode>, f: F, mut program_state: &mut ProgramState) -> Value {
    let mut value: bool = walker(&branchs[0], &mut program_state).extract_bool();
    let mut i: usize = 1;
    while i < branchs.len() {
        let inti: bool = walker(&branchs[i], &mut program_state).extract_bool();
        value = f(value, inti);
        i += 1;
    }
    Value::Bool(value)
}
pub fn walker(astnode: &ASTNode, mut program_state: &mut ProgramState) -> Value {
    match astnode {
        ASTNode::Litteral(i) => i.clone(),
        ASTNode::Operator(operator, branchs) => {
            match operator {
                  Operator::Star => {
                      let mut value: Value = walker(&branchs[0], &mut program_state);
                      let mut j: usize = 1;
                      while j < branchs.len() {
                          value = walker(&branchs[j], &mut program_state);
                          j += 1;
                      }
                      value
                    },
                  Operator::SetTo => {
                      let seto: Value = walker(&branchs[1], &mut program_state);
                      program_state.variables.insert(
                          branchs[0].extract_identifier(),
                          seto.clone()
                      );
                      seto
                  },
                  Operator::Add => domath(branchs, |a, b| a + b, program_state),
                  Operator::Sub => domath(branchs, |a, b| a - b, program_state),
                  Operator::Mul => domath(branchs, |a, b| a * b, program_state),
                  Operator::Div => domath(branchs, |a, b| a / b, program_state),
                  Operator::Mod => domath(branchs, |a, b| a % b, program_state),
                  Operator::And => dologic(branchs, |a, b| a && b, program_state),
                  Operator::Or => dologic(branchs, |a, b| a || b, program_state),
                  Operator::Not => Value::Bool(!(walker(&branchs[0], &mut program_state).extract_bool())),
            }
        },
        ASTNode::Identifier(i) => match program_state.variables.get(i) {
            Some(value) => value.clone(),
            None => panic!("{i} does not exist!"),
        },
        _ => panic!("unexpected node"),
    }
}
