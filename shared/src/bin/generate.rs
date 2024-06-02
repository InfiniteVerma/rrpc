/*
 * 1. read from txt file
 * 2. write to a .rs file
 *
 */
use std::process::Command;
use std::{env, error, fs, process};
// TODO add logging

const FILE_NAME: &str = "gen.rs";
const SUPPORT_DATA_TYPES: [&str; 2] = ["INT", "STRING"];

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

    let out_path = out_dir.join(FILE_NAME);

    println!("Reading TXT file {:?}", file_path);

    let contents =
        fs::read_to_string(file_path).expect(&format!("Could not read {} file", inp_txt_file_path));

    println!("Contents: \n{}", contents);

    let write_output = parse(contents);

    let _ = fs::write(out_path, write_output);

    println!("generate.rs finished. Generated {} file", FILE_NAME);

    let _ = Command::new("rustfmt")
        .args(&[FILE_NAME])
        .output()
        .expect(&format!("Failed to run rustfmt on {}", FILE_NAME));

    let _ = Command::new("sync")
        .output()
        .expect(&format!("Failed to run sync"));

    println!("rustfmt {} finished", FILE_NAME);

    Ok(())
}

// TODO create a struct for LIST<int> and LIST<String>
/*
 * If find ENUM, loop till ENDENUM and every line should have a member of the enum
 *
 */
// TODO improving error, parse using recursive decent? - not needed. we don't have nested types
fn parse(contents: String) -> String {
    let mut write_output = String::new();
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut i = 0;
    let mut enums_defined: Vec<String> = Vec::new();
    let mut structs_defined: Vec<String> = Vec::new();

    write_output.push_str("//gen.rs - This is generated rs file, DO NOT edit manually.\n\n");

    while i < lines.len() {
        let line = lines[i].trim();
        println!("Read line >> {}:{}", i, line);

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
        }

        i += 1;
    }

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
