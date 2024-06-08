use generated::{RpcPacked, Operand};

mod generated {
    include!("generated/client_gen.rs");
}

fn main() {
    let json_inp = RpcPacked 
        { 
            func: String::from("my_func"),
            operands: vec![(String::from("var1"), Operand::Int(1))]
        };
    let client = generated::Client::new(8000);
    let _ = client.send_async(serde_json::to_string::<RpcPacked>(&json_inp).unwrap().as_str());
}
