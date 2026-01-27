use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let endpoint = &args[2];
    let flag: &str;
    match args.get(3) {
        Some(_value) => {
            match args[3].as_str() {
                "--asm" | "-a" => {
                    flag = "--asm";
                }
                "--rustc" | "-r" => {
                    flag = "--rustc";
                }
                _ => {
                    flag = "";
                }
            }
        }
        _ => {
            flag = "";
        }
    }
    let code = std::fs::read_to_string(file_path).expect("Should have been able to read the file");
    compile(&code, endpoint, flag);
}

fn compile(code: &str, endpoint: &str, flag: &str) {
    let lines: Vec<&str> = code.split("\n").collect();
    let mut codebase: Vec<String> = Vec::new();
    for codeline in lines {
        let splitline = codeline.split_whitespace().collect::<Vec<&str>>();
        match splitline[0] {
            // Pretty simple, splitline[1] is simply the thing being printed
            "echo" => {
                let valuething = splitline[1..].join(" ");
                let writeline = format!("println!({});", valuething);
                let finishedline = writeline;
                codebase.push(finishedline);
            }

            // splitline[2] is the name of the variable
            // splitline[3] is values being assigned. this can be either a direct value or follow rust semantics like String::new(), etc etc
            "var" => {
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

            // 1 here is sleep time, measured in milliseconds
            "sleep" => {
                let sleep_line = format!("std::thread::sleep(std::time::Duration::from_millis({}));", splitline[1]);
                let finishedline = sleep_line;
                codebase.push(finishedline);
            }

            // for something like this (and other similar commands), you'll have to use Rust semantics in definitions.
            // using rustc command is also required to take advantage of new crates and the like. it ruins the philosophy a bit but is sadly needed.
            // 1 here is crate path
            "import" => {
                let import_line = format!("use {};", splitline[1]);
                let finishedline = import_line;
                codebase.push(finishedline);
            }

            // Two line process: get input and set variable, then trim variable. to be safe, it also converts to a string.
            // 1 here is variable name
            // 2 is the mandatory expect value, both for unwrap and for errors
            // This process follows for all three input types
            "input" => {
                let input_line = format!("std::io::stdin().read_line(&mut {}).expect(\"{}\"); {} = {}.trim().to_string();", splitline[1], splitline[2..].join(" "), splitline[1], splitline[1]);
                let finishedline = input_line;
                codebase.push(finishedline);
            }

            // Keeps newline. I don't know how useful it'll be, but someone will use it and they will be happy.
            "inputn" => {
                let input_line = format!("std::io::stdin().read_line(&mut {}).expect(\"{}\");", splitline[1], splitline[2..].join(" "));
                let finishedline = input_line;
                codebase.push(finishedline);
            }

            // This trims without conversion. Like I said, someone will be happy when they see it.
            "inputc" => {
                let input_line = format!("std::io::stdin().read_line(&mut {}).expect(\"{}\"); {} = {}.trim();", splitline[1], splitline[2..].join(" "), splitline[1], splitline[1]);
                let finishedline = input_line;
                codebase.push(finishedline);
            }

            "if" => {
                let condition = splitline[1..].join(" ");
                let if_line = format!("if {} {{", condition);
                let finishedline = if_line;
                codebase.push(finishedline);
            }

            "elif" => {
                let condition = splitline[1..].join(" ");
                let elif_line = format!("else if {} {{", condition);
                let finishedline = elif_line;
                codebase.push(finishedline);
            }

            "else" => {
                let else_line = String::from("else {");
                codebase.push(else_line);
            }

            // Originally this was around five different closure keywords, but for simplicity it's now a universal word
            "endblock" => {
                let endif_line = String::from("}");
                codebase.push(endif_line);
            }

            // 1 is variable name
            // 2 is value
            "assign" => {
                let assign_line = format!("{} = {};", splitline[1], splitline[2]);
                let finishedline = assign_line;
                codebase.push(finishedline);
            }

            // For functions, all functions are defined in the main wrapper, seen at the bottom of this code.
            // Normally people wouldn't do this, but because of Aquamarine's design vs. Rust's, that's how control flow works.
            // Closures work about the same way
            // 1 is function name
            // 2..n are parameters [optional]
            "func" => {
                if splitline.len() >= 3 {
                    let func_line = format!("fn {}({}) {{", splitline[1], splitline[2..].join(", "));
                    let finishedline = func_line;
                    codebase.push(finishedline);
                }
                else {
                    let func_line = format!("fn {}() {{", splitline[1]);
                    let finishedline = func_line;
                    codebase.push(finishedline);
                }
            }

            "while" => {
                let condition = splitline[1..].join(" ");
                let while_line = format!("while {} {{", condition);
                let finishedline = while_line;
                codebase.push(finishedline);
            }

            "repeat" => {
                let times = splitline[1];
                let repeat_line = format!("for _ in 0..{} {{", times);
                let finishedline = repeat_line;
                codebase.push(finishedline);
            }

            "loop" => {
                let loop_line = String::from("loop {");
                codebase.push(loop_line);
            }

            "for" => {
                let iterator = splitline[1];
                let range_start = splitline[3];
                let range_end = splitline[5];
                let for_line = format!("for {} in {}..{} {{", iterator, range_start, range_end);
                let finishedline = for_line;
                codebase.push(finishedline);
            }

            "closure" => {
                if splitline.len() >= 3 {
                    let closure_name = splitline[1];
                    let closure_params = splitline[2..].join(", ");
                    let closure_line = format!("let {} = |{}| {{", closure_name, closure_params);
                    let finishedline = closure_line;
                    codebase.push(finishedline);
                }
                else {
                    let closure_name = splitline[1];
                    let closure_line = format!("let {} = || {{", closure_name);
                    let finishedline = closure_line;
                    codebase.push(finishedline);
                }
            }

            // This here can call both functions and closures
            "call" => {
                if splitline.len() >= 3 {
                    let func_name = splitline[1];
                    let func_args = splitline[2..].join(", ");
                    let call_line = format!("{}({});", func_name, func_args);
                    let finishedline = call_line;
                    codebase.push(finishedline);
                }
                else {
                    let func_name = splitline[1];
                    let call_line = format!("{}();", func_name);
                    let finishedline = call_line;
                    codebase.push(finishedline);
                }
            }

            // Inline Rust
            "rustc" => {
                let rustc_line = splitline[1..].join(" ");
                codebase.push(rustc_line);
            }

            // Inline Assembly
            "asm" => {
                let asm_line = format!("unsafe {{ core::arch::asm!({}); }}", splitline[1..].join(" "));
                let finishedline = asm_line;
                codebase.push(finishedline);
            }

            "qdef" => {
                match splitline[1] {
                    // nstr here stands for new string
                    "nstr" => {
                        let qdef_line = format!("let {} = String::new();", splitline[2]);
                        let finishedline = qdef_line;
                        codebase.push(finishedline);
                    }

                    // same for nvec, new vector
                    "nvec" => {
                        let qdef_line = format!("let {}: Vec<_> = Vec::new();", splitline[2]);
                        let finishedline = qdef_line;
                        codebase.push(finishedline);
                    }

                    // ostr stands for owned string, here the string allows for actual value
                    "ostr" => {
                        let qdef_line = format!("let {} = String::from(\"{}\");", splitline[2], splitline[3..].join(" "));
                        let finishedline = qdef_line;
                        codebase.push(finishedline);
                    }

                    // nmap stands for new map
                    "nmap" => {
                        let qdef_line = format!("let {}: std::collections::HashMap<_, _> = std::collections::HashMap::new();", splitline[2]);
                        let finishedline = qdef_line;
                        codebase.push(finishedline);
                    }

                    _ => {
                        panic!("Unknown qdef type: {}", splitline[1]);
                    }
                }
            }

            "//" => {
                let comment_line = format!("// {}", splitline[1..].join(" "));
                codebase.push(comment_line);
            }

            "" => {

            }

            _ => {
                panic!("Unknown command: {}", splitline[0]);
            }
        }
    }
    let transcode = codebase.join("\n");
    let final_code = format!("fn main() {{\n{}\n}}", transcode);
    fs::write(endpoint, final_code).expect("Should have been able to write the file");
    if flag == "--asm" {
        let _rustcabuse = Command::new("rustc").arg("--emit=asm").arg(endpoint).output().expect("Failed to compile the code");
    }
    else if flag == "--rustc" {}
    else {
    let _rustcabuse = Command::new("rustc").arg(endpoint).spawn().expect("Failed to compile the code").wait().expect("Failed to wait on child process");
    }
}