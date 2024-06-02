/*
 * 1. read from txt file
 * 2. write to a .rs file
 *
 */
use std::{fs, env, process, error};
// TODO add logging

#[derive(Debug)]
enum CustomDataTypes {
    ENUM,
    STRUCT,
}

const FILE_NAME: &str = "gen.rs";

fn main() {
    println!("Starting generate.rs");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 
    {
        println!("ERROR: txt file path or out dir not specified! args count: {}", args.len() - 1);
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

    let file_path = current_dir
        .join(inp_txt_file_path);

    let out_path = out_dir 
        .join(FILE_NAME);

    //println!("Reading TXT file {:?}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!("Could not read {} file", inp_txt_file_path));

    println!("Contents: \n{}", contents);

    let write_output = parse(contents);

    let _ = fs::write(out_path, write_output); 

    println!("generate.rs finished. Generated {} file", FILE_NAME);

    Ok(())
}

/*
 * If find ENUM, loop till ENDENUM and every line should have a member of the enum
 *
 *
 */
// TODO improving error, parse using recursive decent? - not needed. we don't have nested types
fn parse(contents: String) -> String {
    let mut write_output = String::new();
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut i = 0;

    write_output.push_str("//gen.rs - This is generated rs file, DO NOT edit manually.\n\n");

    while i < lines.len() {
        let line = lines[i].trim();
        println!("Read line: {}", line);

        if line.starts_with("//") {
            println!("Comment, skipping");
            i += 1;
            continue;
        }

        if line.starts_with("ENUM") {
            let enum_name = line[4..].trim();

            if enum_name.len() < 2 {
                println!("ERROR enum name is not present");
                process::exit(1);
            }

            println!("Found ENUM Block name: {}", enum_name);

            i += 1;
            while i < lines.len() {
                let enum_content = lines[i].trim();
                if enum_content.starts_with("ENDENUM") {
                    println!("Found ENDENUM Block");
                    break;
                }

                println!("Enum content: {}", enum_content);
                i+= 1;
            }
        }

        i+= 1;

    }

    write_output
}
