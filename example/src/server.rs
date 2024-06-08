use generated::{FunctionRegistry, RpcFunction, Operand};
use log::info;
use std::env;

mod generated {
    include!("generated/server_gen.rs");
}

pub struct hello_world_fn;
pub struct area_of_square;

impl RpcFunction for hello_world_fn {
    fn call(&self,operands:Vec<generated::Operand>) -> generated::Operand {

        println!("hello world called with operand: {:#?}", operands);
        generated::Operand::Int(0)
    }
}

impl RpcFunction for area_of_square {
    fn call(&self,operands:Vec<generated::Operand>) -> generated::Operand {
        assert_eq!(operands.len(), 2);
        info!("area_of_square called");

        let mut ans = 1;

        for op in operands {
            match op {
                Operand::Int(var) => ans*= var,
                Operand::Str(_) => panic!("this should not happen"),
            }
        }

        Operand::Int(ans)
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");

    let mut registry = FunctionRegistry::new();
    registry.register("hello_world", hello_world_fn);
    registry.register("area_of_square", area_of_square);

    let server = generated::Server::new(8000, registry);
    let _ = server.start();
}
