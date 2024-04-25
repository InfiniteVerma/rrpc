use serde::Deserialize;

#[derive(Debug)]
pub enum InputType {
    STR,
    JSON,
}

#[derive(Debug, Deserialize)]
pub struct Calculation {
    operand1: i32,
    operand2: i32,
    operator: String,
}
