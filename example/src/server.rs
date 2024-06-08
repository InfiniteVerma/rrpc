use generated::{FunctionRegistry, RpcFunction};

mod generated {
    include!("generated/server_gen.rs");
}

pub struct my_func;

impl RpcFunction for my_func{
    fn call(&self, operands:Vec<generated::Operand>) -> generated::Operand {
        println!("Found operand: {:#?}", operands[0]);
        generated::Operand::Int(0)
    }
}

fn main() {
    let mut registry = FunctionRegistry::new();
    registry.register("my_func", my_func);
    let server = generated::Server::new(8000, registry);
    let _ = server.start();
}
