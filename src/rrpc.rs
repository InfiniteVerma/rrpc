use core::panic;
/*
 * 1. read from txt file
 * 2. write to a .rs file
 *
 */
use std::process::Command;
use std::{env, error, fs, process, str::FromStr};

// TODO add logging
const CLIENT_GEN_FILE: &str = "client_gen.rs";
const SERVER_GEN_FILE: &str = "server_gen.rs";
const SUPPORT_DATA_TYPES: [&str; 2] = ["INT", "STRING"];

// TODO use below instead of above list
#[derive(Debug)]
enum Type {
    INT,
    STRING,
    EMPTY,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INT" => Ok(Type::INT),
            "STRING" => Ok(Type::STRING),
            _ => Err(()),
        }
    }
}

impl Type {
    fn to_rust_type(&self) -> String {
        match self {
            Type::INT => "i64".to_string(),
            Type::STRING => "String".to_string(),
            Type::EMPTY => "()".to_string(),
        }
    }
}

fn main() {
    println!("Starting generate.rs");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "ERROR: txt file path or out dir not specified! args count: {}",
            args.len() - 1
        );
        process::exit(1);
    }

    let inp_txt_file_path = &args[1];
    let out_dir_path = &args[2];

    if let Err(e) = run(inp_txt_file_path, out_dir_path) {
        eprintln!("generate.rs error: {}", e);
        process::exit(1);
    }
}

fn run(inp_txt_file_path: &str, out_dir_path: &str) -> Result<(), Box<dyn error::Error>> {
    println!("file_path: {}", inp_txt_file_path);
    println!("out_dir_path: {}", out_dir_path);

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let out_dir = current_dir.join(out_dir_path);

    let file_path = current_dir.join(inp_txt_file_path);

    let client_out_path = out_dir.join(CLIENT_GEN_FILE);
    let server_out_path = out_dir.join(SERVER_GEN_FILE);

    println!("Reading TXT file {:?}", file_path);

    let contents =
        fs::read_to_string(file_path).expect(&format!("Could not read {} file", inp_txt_file_path));

    println!("Contents: \n{}", contents);

    let client_output = parse_client(contents.as_str());
    let server_output = parse_server(contents.as_str());

    let _ = fs::write(client_out_path, client_output);
    let _ = fs::write(server_out_path, server_output);

    println!("generate.rs finished. Generated {} file", CLIENT_GEN_FILE);

    let _ = Command::new("rustfmt")
        .args(&[CLIENT_GEN_FILE])
        .output()
        .expect(&format!("Failed to run rustfmt on {}", CLIENT_GEN_FILE));

    //let _ = Command::new("sync")
    //    .output()
    //    .expect(&format!("Failed to run sync"));

    println!("rustfmt {} finished", CLIENT_GEN_FILE);

    Ok(())
}

fn parse_client(contents: &str) -> String {
    let mut write_output = String::new();
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut i = 0;
    let mut enums_defined: Vec<String> = Vec::new();
    let mut structs_defined: Vec<String> = Vec::new();
    let mut funcs_defined: Vec<String> = Vec::new();

    //let mut functions_vec: Vec<(&str, &str)> = Vec::new();
    let mut functions_vec: Vec<(String, String)> = Vec::new();

    write_output.push_str("//gen.rs - This is generated rs file, DO NOT edit manually.\n\n");
    write_output.push_str("use serde::{Serialize, Deserialize};\n");
    write_output.push_str("use serde_json::{json};\n");
    write_output.push_str("use std::{fmt, str::FromStr};\n");
    write_output.push_str("use std::io::{self, Read, Write};\n");
    write_output.push_str("use std::net::TcpStream;\n");
    write_output.push_str("use log::{debug, info};\n");

    write_output.push_str(PACK_FUNC_STR);
    write_output.push_str("\n\n");

    while i < lines.len() {
        let line = lines[i].trim();
        println!("Read line >> {}:{}", i, line);

        if line == "\n" || line.len() == 0 {
            i += 1;
            continue;
        }

        if line.starts_with("//") {
            //println!("Comment, skipping");
            i += 1;
            continue;
        }

        if line.starts_with("ENUM") {
            let (enum_str, new_i) = match consume_enum(lines.clone(), i, &mut enums_defined) {
                Ok(x) => x,
                Err(err) => {
                    panic!("Error: {}", err);
                }
            };

            println!("enum_str >> \n---{}\n --, new_i: {}", enum_str, new_i);

            write_output.push_str(&enum_str);
            i = new_i;
        } else if line.starts_with("STRUCT") {
            let (struct_str, new_i) = match consume_struct(lines.clone(), i, &mut structs_defined) {
                Ok(x) => x,
                Err(err) => {
                    panic!("Error: {}", err);
                }
            };

            println!("struct_str >> \n---{}\n --, new_i: {}", struct_str, new_i);

            write_output.push_str(&struct_str);
            i = new_i;
        } else if line.starts_with("FUNCTION") {
            let (func_decl, func_str, new_i) = match consume_function_client(lines.clone(), i, &mut funcs_defined) {
                Ok((func_decl, func_str, new_i)) => (func_decl, func_str, new_i),
                Err(err) => {
                    panic!("Error: {}", err);
                }
            };

            println!("func_str >> \n---{}\n --, new_i: {}", func_str, new_i);
            functions_vec.push((func_decl, func_str));
            i = new_i;
        } else {
            panic!("Unsupported line found: {}", line);
        }

        i += 1;
    }

    write_output.push_str("pub trait RpcFunction {\n");
    for (decl, def) in &functions_vec {
        write_output.push_str(decl.as_str());
    }

    write_output.push_str("}\n");
    write_output.push_str("impl RpcFunction for () {\n");

    for (decl, def) in &functions_vec {
        write_output.push_str(def.as_str());
    }
    write_output.push_str("}\n\n");

    write_output.push_str(DUMMY_CLIENT_MAIN);

    write_output
}

fn parse_server(contents: &str) -> String {

    let mut write_output = String::new();
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut i = 0;
    let mut enums_defined: Vec<String> = Vec::new();
    let mut structs_defined: Vec<String> = Vec::new();
    let mut funcs_defined: Vec<String> = Vec::new();

    write_output.push_str("//gen.rs - This is generated rs file, DO NOT edit manually.\n\n");
    write_output.push_str("use serde::{Serialize, Deserialize};\n");
    write_output.push_str("use std::io::{self, Read};\n");
    write_output.push_str("use std::net::{TcpListener, TcpStream};\n");
    write_output.push_str("use std::collections::HashMap;\n");
    write_output.push_str("use log::info;\n");

    write_output.push_str(UNPACK_FUNC_STR);
    write_output.push_str("\n\n");

    while i < lines.len() {
        let line = lines[i].trim();
        println!("Read line >> {}:{}", i, line);

        if line == "\n" || line.len() == 0 {
            i += 1;
            continue;
        }

        if line.starts_with("//") {
            //println!("Comment, skipping");
            i += 1;
            continue;
        }

        if line.starts_with("ENUM") {
            let (enum_str, new_i) = match consume_enum(lines.clone(), i, &mut enums_defined) {
                Ok(x) => x,
                Err(err) => {
                    panic!("Error: {}", err);
                }
            };

            println!("enum_str >> \n---{}\n --, new_i: {}", enum_str, new_i);

            write_output.push_str(&enum_str);
            i = new_i;
        } else if line.starts_with("STRUCT") {
            let (struct_str, new_i) = match consume_struct(lines.clone(), i, &mut structs_defined) {
                Ok(x) => x,
                Err(err) => {
                    panic!("Error: {}", err);
                }
            };

            println!("struct_str >> \n---{}\n --, new_i: {}", struct_str, new_i);

            write_output.push_str(&struct_str);
            i = new_i;
        } else if line.starts_with("FUNCTION") {
            let (func_str, new_i) = match consume_function_server(lines.clone(), i, &mut funcs_defined) {
                Ok(x) => x,
                Err(err) => {
                    panic!("Error: {}", err);
                }
            };

            println!("func_str >> \n---{}\n --, new_i: {}", func_str, new_i);
            //functions_str.push_str(&func_str);
            i = new_i;
        } else {
            panic!("Unsupported line found: {}", line);
        }

        i += 1;
    }

    write_output.push_str(DUMMY_SERVER_MAIN);

    write_output
}

fn consume_enum(
    lines: Vec<&str>,
    i: usize,
    enums_defined: &mut Vec<String>,
) -> Result<(String, usize), String> {
    let line = lines[i];
    let mut i: usize = i;

    let mut out_str: String = String::new();

    let enum_name = line["ENUM".len()..].trim();

    if enum_name.len() < 2 {
        return Err(format!("ERROR enum name is not present"));
    }

    println!("Found ENUM Block name: {}", enum_name);

    if enums_defined.contains(&enum_name.to_string()) {
        return Err(format!(
            "ERROR: enum {} is already defined in previous enums",
            enum_name
        ));
    }

    out_str.push_str("\n");

    out_str.push_str("#[derive(Debug)]\n");
    out_str.push_str(format!("enum {} {{\n", enum_name).as_str());

    enums_defined.push(enum_name.to_string());

    i += 1;

    let mut found_end: bool = false;
    let mut contains_variant: bool = false;

    while i < lines.len() {
        let enum_content = lines[i].trim();

        // each line should have one word with only alphanumeric characters
        let is_alpha: bool = enum_content.chars().all(|c| c.is_alphanumeric());
        if is_alpha != true {
            return Err(format!(
                "ERROR: line {} contains something other than alphanumeric chars",
                enum_content
            ));
        }

        // first char needs to be an alphabet to satisfy rust syntax rules
        let first_char: char = enum_content.chars().next().unwrap();
        if first_char.is_alphabetic() != true {
            return Err(format!(
                "ERROR: enum variant first char is not alphabetic: {}",
                enum_content
            ));
        }

        if enum_content.starts_with("ENDENUM") {
            println!("Found ENDENUM Block");
            out_str.push_str("}\n");
            found_end = true;
            break;
        }

        contains_variant = true;

        out_str.push_str(format!("  {},\n", enum_content).as_str());

        i += 1;
    }

    if contains_variant != true {
        return Err(format!("ERROR: could not find one variant in enum"));
    }

    if i == lines.len() && found_end != true {
        return Err(format!("ERROR: Could not find ENDENUM"));
    }

    Ok((out_str, i))
}

fn consume_struct(
    lines: Vec<&str>,
    i: usize,
    structs_defined: &mut Vec<String>,
) -> Result<(String, usize), String> {
    let line = lines[i];
    let mut i: usize = i;

    let mut out_str: String = String::new();

    let struct_name = line["STRUCT".len()..].trim();

    if struct_name.len() < 2 {
        return Err(format!("ERROR struct name is not present"));
    }

    println!("Found STRUCT Block name: {}", struct_name);

    if structs_defined.contains(&struct_name.to_string()) {
        return Err(format!(
            "ERROR: struct {} is already defined in previous structs",
            struct_name
        ));
    }

    out_str.push_str("\n");

    out_str.push_str("#[derive(Debug)]\n");
    out_str.push_str(format!("struct {} {{\n", struct_name).as_str());

    structs_defined.push(struct_name.to_string());

    i += 1;

    let mut found_end: bool = false;
    let mut contains_variant: bool = false;

    /*
     * Each line should be of below type:
     *  - <type> <var name>
     *  - ENDSTRUCT
     */
    while i < lines.len() {
        let struct_content = lines[i].trim();

        let tokens_per_line: Vec<&str> = struct_content.split(" ").collect();

        if struct_content.starts_with("ENDSTRUCT") {
            println!("Found ENDSTRUCT Block");
            out_str.push_str("}\n");
            found_end = true;
            break;
        }

        if tokens_per_line.len() != 2 {
            return Err(format!(
                "ERROR: struct parsing failed at line: {}",
                struct_content
            ));
        }

        println!("{:#?}", tokens_per_line);

        // verify data type at this line is supported
        if SUPPORT_DATA_TYPES.contains(&tokens_per_line[0]) != true {
            return Err(format!(
                "ERROR: struct parsing failed at line: {}. Data type not supported",
                struct_content
            ));
        }

        // TODO couple this with support. add a func to convert for each data type
        if tokens_per_line[0] == "INT" {
            out_str.push_str(format!("{}: i64,\n", tokens_per_line[1]).as_str());
        } else if tokens_per_line[0] == "STRING" {
            out_str.push_str(format!("{}: String,\n", tokens_per_line[1]).as_str());
        } else {
            return Err(format!(
                "This should not happen. Check above should catch this"
            ));
        }

        contains_variant = true;

        i += 1;
    }

    if contains_variant != true {
        return Err(format!("ERROR: could not find one variant in struct"));
    }

    if i == lines.len() && found_end != true {
        return Err(format!("ERROR: Could not find ENDSTRUCT"));
    }

    Ok((out_str, i))
}

fn consume_function_client(
    lines: Vec<&str>,
    i: usize,
    funcs_defined: &mut Vec<String>,
) -> Result<(String, String, usize), String> {
    let line = lines[i];
    let mut i: usize = i;

    let mut out_str: String = String::new();
    let mut decl_str: String = String::new();

    let mut params: Vec<(Type, String)> = Vec::new();
    let mut return_type: Type = Type::EMPTY;

    // TODO check for spacing like "FUNCTION test test 2"
    let func_name = line["FUNCTION".len()..].trim();

    if func_name.len() < 2 {
        return Err(format!("ERROR func name is not present"));
    }

    println!("Found FUNCTION Block name: {}", func_name);

    if funcs_defined.contains(&func_name.to_string()) {
        return Err(format!(
            "ERROR: func {} is already defined in previous funcs",
            func_name
        ));
    }

    out_str.push_str("\n");
    decl_str.push_str("\n");

    //out_str.push_str(format!("func {} {{\n", func_name).as_str());

    funcs_defined.push(func_name.to_string());

    i += 1;

    let mut found_end: bool = false;
    let mut contains_variant: bool = false;

    /*
     * Each line should be of below type:
     *  - IN <type> <var name>
     *  - OUT <type> (optional)
     *  - ENDFUNCTION
     *
     *  This loop builds the params list until ENDFUNCTION is reached
     */
    while i < lines.len() {
        let func_content = lines[i].trim();

        let tokens_per_line: Vec<&str> = func_content.split(" ").collect();

        if func_content.starts_with("ENDFUNCTION") {
            println!("Found ENDFUNCTION Block");
            found_end = true;
            break;
        }

        if func_content.starts_with("IN") && tokens_per_line.len() != 3
            || func_content.starts_with("OUT") && tokens_per_line.len() != 2
        {
            return Err(format!(
                "ERROR: func parsing failed at line: {}",
                func_content
            ));
        }

        println!("{:#?}", tokens_per_line);

        // verify data type at this line is supported
        if SUPPORT_DATA_TYPES.contains(&tokens_per_line[1]) != true {
            return Err(format!(
                "ERROR: func parsing failed at line: {}. Data type not supported",
                func_content
            ));
        }

        if tokens_per_line[0] == "IN" {
            // push to params list

            params.push((
                Type::from_str(tokens_per_line[1]).unwrap(),
                tokens_per_line[2].to_string(),
            ));
        } else if tokens_per_line[0] == "OUT" {
            // params is done. this specifies return type. peek next line here only

            return_type = Type::from_str(tokens_per_line[1]).unwrap();

            // verify next line is ENDFUNCTION
            // TODO idk rethink below
            i += 1;
            let func_content = lines[i].trim();
            if func_content.contains("ENDFUNCTION") != true {
                return Err(format!(
                    "ERROR: func parsing failed at line: {}. ENDFUNCTION not at next line to OUT",
                    func_content
                ));
            }
            i -= 1;
        } else {
            return Err(format!(
                "ERROR: func parsing failed at line: {}",
                func_content
            ));
        }

        contains_variant = true;

        i += 1;
    }

    if contains_variant != true {
        return Err(format!("ERROR: could not find one variant in func"));
    }

    if i == lines.len() && found_end != true {
        return Err(format!("ERROR: Could not find ENDFUNCTION"));
    }

    decl_str.push_str(format!("  fn {} (client: &Client, ", func_name).as_str());
    out_str.push_str(format!("  fn {} (client: &Client, ", func_name).as_str());

    for (type_enum, var) in &params {
        decl_str.push_str(format!(" {}: {}", var, Type::to_rust_type(type_enum)).as_str());
        decl_str.push_str(", ");

        out_str.push_str(format!(" {}: {}", var, Type::to_rust_type(type_enum)).as_str());
        out_str.push_str(", ");
    }

    decl_str.push_str(");\n\n");
    out_str.push_str(")");
    out_str.push_str(format!(" -> {} {{\n", Type::to_rust_type(&return_type)).as_str());
    out_str.push_str(format!("    let func_name: String = String::from_str(\"{}\").unwrap();\n", func_name).as_str());

    out_str.push_str("    let operands = vec![");
    for (type_enum, var) in &params {

        let operand_type = match type_enum {
            Type::INT => format!("Operand::Int({})", var),
            Type::STRING => format!("Operand::Str({})", var),
            Type::EMPTY => panic!("This should not happen"),
        };

        out_str.push_str(format!("( String::from_str(\"{}\").unwrap(), {} ),\n ", var, operand_type).as_str());
    }

    out_str.push_str("];\n\n");

    out_str.push_str("    let json_str = pack(func_name, operands);\n\n");
    //out_str.push_str("    let client = Client::new(8080);\n\n");
    out_str.push_str("    client.send_async(json_str.as_str()).unwrap();\n");
    out_str.push_str("  }\n");

    Ok((decl_str, out_str, i))
}

fn consume_function_server(
    lines: Vec<&str>,
    i: usize,
    funcs_defined: &mut Vec<String>,
) -> Result<(String, usize), String> {
    let line = lines[i];
    let mut i: usize = i;

    let mut out_str: String = String::new();

    let mut params: Vec<(Type, String)> = Vec::new();
    let mut return_type: Type = Type::EMPTY;

    // TODO check for spacing like "FUNCTION test test 2"
    let func_name = line["FUNCTION".len()..].trim();

    if func_name.len() < 2 {
        return Err(format!("ERROR func name is not present"));
    }

    println!("Found FUNCTION Block name: {}", func_name);

    if funcs_defined.contains(&func_name.to_string()) {
        return Err(format!(
            "ERROR: func {} is already defined in previous funcs",
            func_name
        ));
    }

    out_str.push_str("\n");

    //out_str.push_str(format!("func {} {{\n", func_name).as_str());

    funcs_defined.push(func_name.to_string());

    i += 1;

    let mut found_end: bool = false;
    let mut contains_variant: bool = false;

    /*
     * Each line should be of below type:
     *  - IN <type> <var name>
     *  - OUT <type> (optional)
     *  - ENDFUNCTION
     *
     *  This loop builds the params list until ENDFUNCTION is reached
     */
    while i < lines.len() {
        let func_content = lines[i].trim();

        let tokens_per_line: Vec<&str> = func_content.split(" ").collect();

        if func_content.starts_with("ENDFUNCTION") {
            println!("Found ENDFUNCTION Block");
            found_end = true;
            break;
        }

        if func_content.starts_with("IN") && tokens_per_line.len() != 3
            || func_content.starts_with("OUT") && tokens_per_line.len() != 2
        {
            return Err(format!(
                "ERROR: func parsing failed at line: {}",
                func_content
            ));
        }

        println!("{:#?}", tokens_per_line);

        // verify data type at this line is supported
        if SUPPORT_DATA_TYPES.contains(&tokens_per_line[1]) != true {
            return Err(format!(
                "ERROR: func parsing failed at line: {}. Data type not supported",
                func_content
            ));
        }

        if tokens_per_line[0] == "IN" {
            // push to params list

            params.push((
                Type::from_str(tokens_per_line[1]).unwrap(),
                tokens_per_line[2].to_string(),
            ));
        } else if tokens_per_line[0] == "OUT" {
            // params is done. this specifies return type. peek next line here only

            return_type = Type::from_str(tokens_per_line[1]).unwrap();

            // verify next line is ENDFUNCTION
            // TODO idk rethink below
            i += 1;
            let func_content = lines[i].trim();
            if func_content.contains("ENDFUNCTION") != true {
                return Err(format!(
                    "ERROR: func parsing failed at line: {}. ENDFUNCTION not at next line to OUT",
                    func_content
                ));
            }
            i -= 1;
        } else {
            return Err(format!(
                "ERROR: func parsing failed at line: {}",
                func_content
            ));
        }

        contains_variant = true;

        i += 1;
    }

    if contains_variant != true {
        return Err(format!("ERROR: could not find one variant in func"));
    }

    if i == lines.len() && found_end != true {
        return Err(format!("ERROR: Could not find ENDFUNCTION"));
    }

    out_str.push_str(format!("  fn {} (", func_name).as_str());

    // fn test(var: int
    for (type_enum, var) in &params {
        out_str.push_str(format!(" {}: {}", var, Type::to_rust_type(type_enum)).as_str());
        out_str.push_str(", ");
    }

    out_str.push_str(")");
    out_str.push_str(format!(" -> {} {{\n", Type::to_rust_type(&return_type)).as_str());
    out_str.push_str(format!("    let func_name: String = String::from_str(\"{}\").unwrap();\n", func_name).as_str());

    out_str.push_str("    let operands = vec![");
    for (type_enum, var) in &params {

        let operand_type = match type_enum {
            Type::INT => format!("Operand::Int({})", var),
            Type::STRING => format!("Operand::Str({})", var),
            Type::EMPTY => panic!("This should not happen"),
        };

        out_str.push_str(format!("( String::from_str(\"{}\").unwrap(), {} ),\n ", var, operand_type).as_str());
    }

    //out_str.push_str("];\n\n");
    Ok((out_str, i))
}

const PACK_FUNC_STR: &str = r#"
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcPacked {
    pub func: String,
    pub operands: Vec<(String, Operand)>,
}

pub struct Client {
    //input_type: InputType; // TODO
    port: u32,
}

impl Client {

    pub fn new(port: u32) -> Self {
        env_logger::init();
        Client { port }
    }

    /// Sends a ASYNC command to the server
    ///
    /// This is a nonblocking call. Client doesn't wait for a response.
    pub fn send_async(&self, expression: &str) -> Result<(), io::Error> {

        let address = format!("127.0.0.1:{}", self.port);

        let mut stream = TcpStream::connect(address)?;

        stream.write_all(expression.as_bytes())?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operand {
    Int(i64),
    Str(String),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Operand::Int(value) => write!(f, "{}", value),
           Operand::Str(value) => write!(f, "\"{}\"", value),
       }
    }
}

fn pack(func: String, operands: Vec<(String, Operand)>) -> String {

    let json_data = RpcPacked {
        func,
        operands,
    };

    serde_json::to_string(&json_data).unwrap()
}
"#;

const UNPACK_FUNC_STR: &str = r#"

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcPacked { // TODO should not be pub
    pub func: String,
    pub operands: Vec<(String, Operand)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operand {
    Int(i64),
    Str(String),
}

pub trait RpcFunction {
    fn call(&self, operands: Vec<Operand>) -> Operand;
}

// Box -> smart pointer for a heap allocated obj
// dyn -> dynamic dispatch (Why needed?)
type RpcFunctionBox = Box<dyn RpcFunction>;

/// stores the functions defined by user
/// user needs to define one for it to work
pub struct FunctionRegistry {
    functions: HashMap<String, RpcFunctionBox>
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: &str, func: F)
    where
        F: RpcFunction + 'static,
    {
        self.functions.insert(name.to_string(), Box::new(func));
    }

    pub fn get(&self, name: &str) -> Option<&RpcFunctionBox> {
        self.functions.get(name)
    }
}

/// Macro to build the registery easily
#[macro_export]
macro_rules! register_functions {
    ($registery:expr, $($name:expr => $func:expr), *) => {
       $(
           $registery.register($name, $func);
       )*
    };
}

pub struct Server {
    // input_type: InputType,
    port: u32,
    registry: FunctionRegistry,
}

impl Server {
    pub fn new(port: u32, registry: FunctionRegistry) -> Self {
        env_logger::init();
        Server { port, registry }
    }

    pub fn start(&self) -> io::Result<()> {
        info!("server BEGIN");

        let address = format!("127.0.0.1:{}", self.port);

        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            info!("found new stream");

            let stream = stream?;

            self.process_request(stream);
        }

        Ok(())
    }

    /// Input: TcpStream
    ///
    /// Get json from it, parse it, call the function
    ///
    /// {
    ///   "fn": func_name,
    ///   "operands": json_operands 
    /// }
    ///
    /// user will 'register' their functions when they use the pkg. below func would search btw
    /// registered func. If it finds, it processes otherwise ignores it (but logs it)
    pub fn process_request(&self, mut stream: TcpStream) {
        info!("process_request BEGIN");

        let mut buffer = [0; 512];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                info!("Bytes read: {}", bytes_read);

                let mut json_in_str = String::from_utf8_lossy(&buffer[..]).into_owned();

                json_in_str = String::from(json_in_str.trim().trim_matches('\0'));

                info!("{}", json_in_str);

                let json_val: RpcPacked = serde_json::from_str::<RpcPacked>(json_in_str.as_str()).unwrap();

                match self.registry.get(json_val.func.as_str()) {
                    Some(func) => {
                        let ans = func.call(
                            json_val.operands
                            .into_iter()
                            .map(|(_, value)| value)
                            .collect()
                            );
                        info!("answer: {:#?}", ans);
                    }
                    None => {
                        info!("ERROR: {} not found in registry", json_val.func);
                    }
                }
            },
            Err(err) => {
                info!("{}", err);
            }
        }

    }
}

"#;

const DUMMY_CLIENT_MAIN: &str = r#"
// -----------------------------------------------
// Dummy client main. For Testing. TODO comment via a flag?
// -----------------------------------------------
//fn main() {
//    let client = Client::new(8000);
//    <() as RpcFunction>::my_func(client, 1);
//}
"#;

const DUMMY_SERVER_MAIN: &str = r#"
// -----------------------------------------------
// Dummy server main. For Testing. TODO comment via a flag?
// -----------------------------------------------
//pub struct addAsyncFunc; 
//
//impl RpcFunction for addAsyncFunc {
//    fn call(&self, operands: Vec<Operand>) -> Operand {
//        let mut answer = 0;
//        for operand in operands {
//            match operand {
//                Operand::Int(var) => answer += var,
//                Operand::Str(_) => panic!("Wrong param passed"),
//            }
//        }
//        Operand::Int(answer)
//    }
//}
//
//fn main() {
//    // register my functions
//    let mut registry = FunctionRegistry::new();
//    registry.register("addAsyncFunc", addAsyncFunc);
//    
//    let server = Server::new(8000, registry);
//    let _ = server.start();
//}
"#;
