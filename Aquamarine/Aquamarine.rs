use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let endpoint = &args[2];
    let code = std::fs::read_to_string(file_path).expect("Should have been able to read the file");
    compile(&code, endpoint);
}

fn compile(code: &str, endpoint: &str) {
    let lines: Vec<&str> = code.split("\n").collect();
    let mut codebase: Vec<String> = Vec::new();
    for codeline in lines {
        let splitline = codeline.split_whitespace().collect::<Vec<&str>>();
        if splitline[0] == "echo" {
            let valuething = splitline[1..].join(" ");
            let writeline = format!("println!(\"{}\");", valuething);
            let finishedline = writeline;
            codebase.push(finishedline);
        }

        else if splitline[0] == "var" {
            if splitline[1] == "mut" {
                let var_declaration = format!("let mut {} = {};", splitline[2], splitline[3]);
                let finishedline = var_declaration;
                codebase.push(finishedline);
            }

            else if splitline[1] == "!mut" {
                let var_declaration2 = format!("let {} = {};", splitline[2], splitline[3]);
                let finishedline = var_declaration2;
                codebase.push(finishedline);
            }
        }
    }
    let transcode = codebase.join("\n");
    let final_code = format!("fn main() {{\n{}\n}}", transcode);
    fs::write(endpoint, final_code).expect("Should have been able to write the file");
}