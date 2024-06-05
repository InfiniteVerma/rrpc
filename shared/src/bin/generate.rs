use core::panic;
/*
 * 1. read from txt file
 * 2. write to a .rs file
 *
 */
use std::process::Command;
use std::{env, error, fs, process, str::FromStr};

use serde_json::{json, to_string};
// TODO add logging

const CLIENT_GEN_FILE: &str = "client_gen.rs";
// TODO implement server gen
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

    //const FUNC: &str = "test_func";
    //let vec_try = Vec::from());
    //let mut vec_try = Vec::new();
    //vec_try.push((String::from_str("var1").unwrap(), Operand::Int(2)));
    //println!("{}", pack(FUNC, vec_try));
}

fn run(inp_txt_file_path: &str, out_dir_path: &str) -> Result<(), Box<dyn error::Error>> {
    println!("file_path: {}", inp_txt_file_path);
    println!("out_dir_path: {}", out_dir_path);

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let out_dir = current_dir.join(out_dir_path);

    let file_path = current_dir.join(inp_txt_file_path);

    let out_path = out_dir.join(CLIENT_GEN_FILE);

    println!("Reading TXT file {:?}", file_path);

    let contents =
        fs::read_to_string(file_path).expect(&format!("Could not read {} file", inp_txt_file_path));

    println!("Contents: \n{}", contents);

    let write_output = parse(contents);

    let _ = fs::write(out_path, write_output);

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

fn parse(contents: String) -> String {
    let mut write_output = String::new();
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut i = 0;
    let mut enums_defined: Vec<String> = Vec::new();
    let mut structs_defined: Vec<String> = Vec::new();
    let mut funcs_defined: Vec<String> = Vec::new();

    let mut functions_str: String = String::new();

    write_output.push_str("//gen.rs - This is generated rs file, DO NOT edit manually.\n\n");
    write_output.push_str("use serde_json::{json};\n");
    write_output.push_str("use std::{fmt, str::FromStr};\n");

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
            let (func_str, new_i) = match consume_function(lines.clone(), i, &mut funcs_defined) {
                Ok(x) => x,
                Err(err) => {
                    panic!("Error: {}", err);
                }
            };

            println!("func_str >> \n---{}\n --, new_i: {}", func_str, new_i);
            functions_str.push_str(&func_str);
            i = new_i;
        } else {
            panic!("Unsupported line found: {}", line);
        }

        i += 1;
    }

    write_output.push_str("pub trait RpcFunctions {\n"); // TODO make this name a parameter?
    write_output.push_str(&functions_str);
    write_output.push_str("}\n\n");

    write_output.push_str(PACK_FUNC_STR);

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

fn consume_function(
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
            Type::INT => "Operand::Int(
            Type::STRING => "String",
            Type::EMPTY => panic!("This should not happen"),
        };

        out_str.push_str(format!("( String::from_str(\"{}\").unwrap(), {} ),\n ", var, operand_type).as_str());
    }

    out_str.push_str("];\n");

    out_str.push_str("    let json_str = pack(func_name, operands);\n");
    out_str.push_str("  }\n");

    Ok((out_str, i))
}

const PACK_FUNC_STR: &str = r#"

enum Operand {
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

fn pack(func_name: String, operands: Vec<(String, Operand)>) -> String {

    let json_operands: Vec<_> = operands
        .iter()
        .map(|(key, value)| json!({key: value.to_string()}))
        .collect();

    let json_data = json!({
        "fn": func_name,
        "operands": json_operands 
    });

    serde_json::to_string(&json_data).unwrap()
}
"#;

const CALL_PACK_STR : &str = r#"

    let vec_try = Vec::from());
    let mut vec_try = Vec::new();
    vec_try.push();

    let json_string: String = pack(func_name, vec!);
"#;

