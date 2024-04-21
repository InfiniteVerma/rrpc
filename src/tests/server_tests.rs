use rrpc::server;

#[test]
fn test_rpc_server() {
    // Start the server
    match server::start() {
        Ok(_) => {
            // Test server functionality here
        }
        Err(err) => {
            panic!("Failed to start RPC server: {}", err);
        }
    }
}

