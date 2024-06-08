use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum InputType {
    STR,
    JSON,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum RequestType {
    SYNC,
    ASYNC,
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
    pub request_type: RequestType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub cmd: String,
    pub request_type: RequestType,
}

pub const KILL_JSON: &str = "
{
    \"type\": \"command\",
    \"cmd\": \"KILL\",
    \"request_type\": \"SYNC\"
}";

pub const KILL_STR: &str = "KILL";

#[derive(Debug, Deserialize, Serialize)]
struct TestWrapper {
    request: MsgContentTypes,
    expected_response: String,
}
