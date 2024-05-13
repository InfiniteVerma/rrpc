use shared::shared::{Calculation, Command, InputType, MsgContentTypes, RequestType};
use log::{debug, info};
use std::io::{self};

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum RequestResult {
    ParsedStr(String),
    ParsedStrCommand(String),
    ParsedJSONCalculation(Calculation),
    ParsedJSONCommand(Command),
    Error(io::Error),
}

impl RequestResult {
    pub fn evaluate(&self, request_type: RequestType) -> Result<String, String> { 
        let response = match request_type {
            RequestType::SYNC => {},
            RequestType::ASYNC => {},
        }
    }
}

pub fn evaluate_expr_json(expr: &Calculation) -> Result<String, String> {
    // TODO why is this needed?
    //let expression = expression.trim().trim_matches('\0');

    info!("evaluate_expr_json BEGIN {:#?}", expr);

    let op_char = expr.operator.chars().next().unwrap();

    //// TODO make this in below function too
    let op: Operation = match op_char {
        '*' => Operation::Multiply,
        '+' => Operation::Add,
        '-' => Operation::Subtract,
        '/' => Operation::Divide,
        _ => return Err(format!("Operand {:#?} is not supported", op_char)),
    };

    return Ok(evaluate(
        &expr.operand1.to_string(),
        &expr.operand2.to_string(),
        op,
    ));
}

// TODO call diff func on json?
pub fn evaluate_expr(expression: &str) -> Result<String, String> {
    let expression = expression.trim().trim_matches('\0');

    if expression == "KILL" {
        return Ok(format!("KILL"));
    } else if let Some(index) = expression.find(|c: char| !c.is_numeric()) {
        debug!("{:#?}", index);
        let (operand1, operand2) = expression.split_at(index);

        let op = operand2.chars().next().unwrap();

        let operand2 = &operand2[1..];

        debug!("{:#?}", expression.chars().nth(index));

        // TODO make this in below function too
        let op: Operation = match op {
            '*' => Operation::Multiply,
            '+' => Operation::Add,
            '-' => Operation::Subtract,
            '/' => Operation::Divide,
            _ => return Err(format!("Operand {:#?} is not supported", op)),
        };

        return Ok(evaluate(operand1, operand2, op));
    }

    Err(format!("Final return Invalid expr. Expect <opr1>+<opr2>"))
}

fn evaluate(operand1: &str, operand2: &str, operation: Operation) -> String {
    let (operand1, operand2) = (
        operand1.parse::<i32>().unwrap(),
        operand2.parse::<i32>().unwrap(),
    );

    info!(
        "Operating {:#?} {:#?} with operand {:#?}",
        operand1, operand2, operation
    );

    let result = match operation {
        Operation::Add => Ok(operand1 + operand2),
        Operation::Subtract => Ok(operand1 - operand2),
        Operation::Multiply => Ok(operand1 * operand2),
        Operation::Divide => match operand2 {
            0 => Err("Division by zero"),
            _ => Ok(operand1 / operand2),
        },
    };

    match result {
        Ok(ans) => ans.to_string(),
        Err(err) => err.to_string(),
    }
}

