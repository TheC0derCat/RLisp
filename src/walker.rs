use crate::lexer::*;
use crate::parser::*;
use std::collections::HashMap;
use std::io;
use std::io::Write;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramState {
    pub variables: HashMap<String, Value>,
}
pub fn domath<F: Fn(i32, i32) -> i32>(
    branchs: &Vec<ASTNode>,
    f: F,
    mut program_state: &mut ProgramState,
) -> Value {
    let mut value: i32 = walker(&branchs[0], &mut program_state).extract_int();
    let mut i: usize = 1;
    while i < branchs.len() {
        let inti: i32 = walker(&branchs[i], &mut program_state).extract_int();
        value = f(value, inti);
        i += 1;
    }
    Value::Int(value)
}
pub fn dologic<F: Fn(bool, bool) -> bool>(
    branchs: &Vec<ASTNode>,
    f: F,
    mut program_state: &mut ProgramState,
) -> Value {
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
        ASTNode::LambdaCall(i, args) => match program_state.variables[i].clone() {
            Value::Lambda(ref extracted_lambda, ref closure_program_state, names) => {
                let mut new_program_state = closure_program_state.clone();
                let mut i: usize = 0;
                while i < args.len() {
                    new_program_state
                        .variables
                        .insert(names[i].clone(), walker(&args[i], &mut program_state));
                    i += 1;
                }
                //new_program_state
                //    .variables
                //    .insert("arg".to_string(), walker(&*arg, &mut program_state));
                walker(&extracted_lambda, &mut new_program_state)
            }
            _ => panic!("cant run non lambda as lambda"),
        },
        ASTNode::Operator(operator, branchs) => match operator {
            Operator::List => {
                let mut the_list: Vec<Value> = Vec::new();
                for branch in branchs {
                    the_list.push(walker(&branch, &mut program_state));
                }
                Value::List(the_list)
                // Value::List(branchs.into_iter().map(|x| walker(x, &mut program_state)).rev().collect())
            },
            Operator::Nth => {
                let the_list: Vec<Value> = walker(&branchs[0], &mut program_state).extract_list();
                let index: usize = walker(&branchs[1], &mut program_state).extract_int() as usize;
                the_list[index].clone()
            },
            Operator::Setnth => {
                let mut the_list: Vec<Value> = walker(&branchs[0], &mut program_state).extract_list();
                let index: usize = walker(&branchs[1], &mut program_state).extract_int() as usize;
                let setto: Value = walker(&branchs[2], &mut program_state);
                the_list[index] = setto;
                Value::List(the_list)
            },
            Operator::Len => Value::Int(walker(&branchs[0], &mut program_state).extract_list().len() as i32),
            Operator::Lambda => {
                let mut argnames: Vec<String> = Vec::new();
                for branch in &branchs[1..] {
                    match branch {
                        ASTNode::Identifier(id) => argnames.push(id.to_string()),
                        _ => panic!("cant use a non identifier as a argument name!"),
                    }
                }
                Value::Lambda(
                    Box::new(branchs[0].clone()),
                    program_state.clone(),
                    argnames,
                )
            }
            Operator::If => {
                if walker(&branchs[0], &mut program_state).extract_bool() {
                    walker(&branchs[1], &mut program_state)
                } else {
                    walker(&branchs[2], &mut program_state)
                }
            }
            Operator::Loop => {
                let loop_max: i32 = walker(&branchs[0], &mut program_state).extract_int();
                let mut j: usize = 0;
                while j < loop_max.try_into().unwrap() {
                    walker(&branchs[1], &mut program_state);
                    j += 1;
                }
                Value::Int(loop_max)
            }
            Operator::Input => {
                let mut buffer = String::new();
                io::stdout().flush();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("failed to readline");
                Value::Str(buffer.trim().to_string())
            }
            Operator::Output => {
                let mut value: Value = walker(&branchs[0], &mut program_state);
                let mut j: usize = 0;
                while j < branchs.len() {
                    value = walker(&branchs[j], &mut program_state);
                    match value {
                        Value::Int(ref i) => println!("{i}"),
                        Value::Str(ref i) => println!("{i}"),
                        Value::Bool(ref i) => println!("{i}"),
                        Value::Lambda(ref i, _, _) => println! {"trying to print a lambda? lolz",
                        },
                        Value::Null => println!("Null"),
                        Value::List(ref i) => println!("{:?}", i),
                    }
                    j += 1;
                }
                value
            }
            Operator::Star => {
                let mut value: Value = walker(&branchs[0], &mut program_state);
                let mut j: usize = 1;
                while j < branchs.len() {
                    value = walker(&branchs[j], &mut program_state);
                    j += 1;
                }
                value
            }
            Operator::SetTo => {
                let seto: Value = walker(&branchs[1], &mut program_state);
                program_state
                    .variables
                    .insert(branchs[0].extract_identifier(), seto.clone());
                seto
            }
            Operator::Exit => std::process::exit(0),
            Operator::Equality => {
                let mut boolean: bool = true;
                let mut value: Value = walker(&branchs[0], &mut program_state);
                let mut j: usize = 1;
                while j < branchs.len() {
                    let new_value = walker(&branchs[j], &mut program_state);
                    boolean = value == new_value;
                    value = walker(&branchs[j], &mut program_state);
                    j += 1;
                }
                Value::Bool(boolean)
            }
            Operator::Add => domath(branchs, |a, b| a + b, program_state),
            Operator::Sub => domath(branchs, |a, b| a - b, program_state),
            Operator::Mul => domath(branchs, |a, b| a * b, program_state),
            Operator::Div => domath(branchs, |a, b| a / b, program_state),
            Operator::Mod => domath(branchs, |a, b| a % b, program_state),
            Operator::And => dologic(branchs, |a, b| a && b, program_state),
            Operator::Or => dologic(branchs, |a, b| a || b, program_state),
            Operator::Not => Value::Bool(!(walker(&branchs[0], &mut program_state).extract_bool())),
        },
        ASTNode::Identifier(i) => match program_state.variables.get(i) {
            Some(value) => value.clone(),
            None => panic!("{i} does not exist!"),
        },
        _ => Value::Null,
    }
}
