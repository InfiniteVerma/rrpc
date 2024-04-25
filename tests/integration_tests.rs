#[cfg(test)]
mod tests {

    use rrpc::shared::{InputType, Calculation};
    use rrpc::{client::Client, server::Server};
    use serde_json::{Result, Value};
    use std::{fs, env};

    //#[test]
    //fn test_rpc_integration_str() {
    //    let test_cases = vec![
    //        ("2+4", "6"),
    //        ("2+5", "7"),
    //        ("22+15", "37"),
    //        // subtract
    //        ("5-2", "3"),
    //        // multiply
    //        ("5*2", "10"),
    //        // divide
    //        ("5/2", "2"),
    //        ("5/0", "Division by zero"),
    //    ];

    //    let _server_handle = std::thread::spawn(|| {
    //        let server = Server::new(InputType::STR, 8080);
    //        server.start().unwrap();
    //    });

    //    std::thread::sleep(std::time::Duration::from_secs(1));

    //    let client = Client::new(InputType::STR, 8080);

    //    for (input, expected_output) in test_cases {
    //        let resp = client.send_sync(input).unwrap();

    //        assert_eq!(resp, expected_output);
    //    }

    //    client.kill().unwrap();

    //    _server_handle.join().unwrap();
    //}

    #[test]
    fn test_rpc_integration_json() {

        let current_dir = env::current_dir().expect("Failed to get current directory");

        let file_path = current_dir
            .join("tests")
            .join("test.json");

        println!("{:?}", file_path);

        // Read the contents of the file
        let contents = fs::read_to_string(file_path).expect("Could not read test.json file");

        let request: Calculation = serde_json::from_str(&contents)
            .expect("Failed to deserialize JSON into calculation struct");

        let _server_handle = std::thread::spawn(|| {
            let server = Server::new(InputType::JSON, 8080);
            server.start().unwrap();
        });

        std::thread::sleep(std::time::Duration::from_secs(1));

        let client = Client::new(InputType::JSON, 8080);

        println!("Sending to socket");

        let resp = client.send_sync(&request).unwrap();

        assert_eq!(resp, "3");

        client.kill().unwrap();

        _server_handle.join().unwrap();
    }
}
