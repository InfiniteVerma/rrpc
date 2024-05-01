use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum InputType {
    STR,
    JSON,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MsgContentTypes {
    #[serde(rename = "operation")]
    Type1(Calculation),
    #[serde(rename = "command")]
    Type2(Command),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Calculation {
    pub operand1: i32,
    pub operand2: i32,
    pub operator: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub cmd: String,
}

pub const KILL_JSON: &str = "
{
    \"type\": \"command\",
    \"cmd\": \"KILL\"
}";

pub const KILL_STR: &str = "KILL";
