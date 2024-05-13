#[cfg(test)]
mod tests {

    use shared::shared::{Calculation, InputType, MsgContentTypes};

    // Import Client and Server structs from client and server modules
    use client::client::Client;
    use server::server::Server;

    use serde::{Deserialize, Serialize};
    use std::{fs, env};

    #[derive(Debug, Deserialize, Serialize)]
    struct TestWrapper {
        request: MsgContentTypes,
        expected_response: String 
    }

    fn generate_test_cases() -> Vec<(&'static str, &'static str)> {
        vec![
            ("2+4", "6"),
            ("2+5", "7"),
            ("22+15", "37"),
            // subtract
            ("5-2", "3"),
            // multiply
            ("5*2", "10"),
            // divide
            ("5/2", "2"),
            ("5/0", "Division by zero"),
        ]
    }

    #[test]
    fn test_rpc_integration_str_sync() {
        let test_cases = generate_test_cases();

        let _server_handle = std::thread::spawn(|| {
            let server = Server::new(InputType::STR, 8080);
            server.start().unwrap();
        });

        std::thread::sleep(std::time::Duration::from_secs(1));

        let client = Client::new(InputType::STR, 8080);

        for (input, expected_output) in test_cases {
            let resp = client.send_sync(input).unwrap();

            assert_eq!(resp, expected_output);
        }

        client.kill().unwrap();

        _server_handle.join().unwrap();
    }

    //#[test]
    //fn test_rpc_integration_str_async() {
    //    let test_cases = generate_test_cases();

    //    let _server_handle = std::thread::spawn(|| {
    //        let server = Server::new(InputType::STR, 8080);
    //        server.start().unwrap();
    //    });

    //    std::thread::sleep(std::time::Duration::from_secs(1));

    //    let client = Client::new(InputType::STR, 8080);

    //    for (input, expected_output) in test_cases {
    //        client.send_async(input).unwrap();
    //    }

    //    client.kill().unwrap();

    //    _server_handle.join().unwrap();
    //}

    //#[test]
    //fn test_rpc_integration_json_sync() {

    //    let current_dir = env::current_dir().expect("Failed to get current directory");

    //    let file_path = current_dir
    //        .join("data")
    //        .join("test.json");

    //    println!("{:?}", file_path);

    //    // Read the contents of the file
    //    let contents = fs::read_to_string(file_path).expect("Could not read test.json file");

    //    let test_requests: Vec<TestWrapper> = serde_json::from_str(&contents)
    //        .expect("Failed to deserialize JSON into calculation struct");

    //    let _server_handle = std::thread::spawn(|| {
    //        let server = Server::new(InputType::JSON, 8080);
    //        server.start().unwrap();
    //    });

    //    std::thread::sleep(std::time::Duration::from_secs(1));

    //    let client = Client::new(InputType::JSON, 8080);

    //    for test_request in test_requests {
    //        println!("Sending to socket");

    //        let request = serde_json::to_string(&test_request.request).unwrap();

    //        let resp = client.send_sync(&request).unwrap();

    //        assert_eq!(resp, test_request.expected_response);
    //    }

    //    client.kill().unwrap();

    //    _server_handle.join().unwrap();
    //}

}

fn main() {}
