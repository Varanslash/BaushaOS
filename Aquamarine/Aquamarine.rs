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
    let mut codebase: Vec<String> = Vec::new();
    let mut linecounter = 0;
    for codeline in code.lines() {
        let splitline = codeline.split_whitespace().collect::<Vec<&str>>();
        match splitline.get(0) {
            Some(_value) => {
                match splitline[0] {
                    // Pretty simple, splitline[1] is simply the thing being printed
                    "echo" => {
                        let valuething = splitline[1..].join(" ");
                        let writeline = format!("println!({});", valuething);
                        let finishedline = writeline;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    // splitline[2] is the name of the variable
                    // splitline[3] is values being assigned. this can be either a direct value or follow rust semantics like String::new(), etc etc
                    "var" => {
                        if splitline[1] == "mut" {
                            let var_declaration = format!("let mut {} = {};", splitline[2], splitline[3]);
                            let finishedline = var_declaration;
                            codebase.push(finishedline);
                            linecounter += 1
                        }

                        else if splitline[1] == "!mut" {
                            let var_declaration2 = format!("let {} = {};", splitline[2], splitline[3]);
                            let finishedline = var_declaration2;
                            codebase.push(finishedline);
                            linecounter += 1
                        }
                    }

                    // 1 here is sleep time, measured in milliseconds
                    "sleep" => {
                        let sleep_line = format!("std::thread::sleep(std::time::Duration::from_millis({}));", splitline[1]);
                        let finishedline = sleep_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    // for something like this (and other similar commands), you'll have to use Rust semantics in definitions.
                    // using rustc command is also required to take advantage of new crates and the like. it ruins the philosophy a bit but is sadly needed.
                    // 1 here is crate path
                    "import" => {
                        let import_line = format!("use {};", splitline[1]);
                        let finishedline = import_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    // Two line process: get input and set variable, then trim variable. to be safe, it also converts to a string.
                    // 1 here is variable name
                    // 2 is the mandatory expect value, both for unwrap and for errors
                    // This process follows for all three input types
                    "input" => {
                        let input_line = format!("std::io::stdin().read_line(&mut {}).expect(\"{}\"); {} = {}.trim().to_string();", splitline[1], splitline[2..].join(" "), splitline[1], splitline[1]);
                        let finishedline = input_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    // Keeps newline. I don't know how useful it'll be, but someone will use it and they will be happy.
                    "inputn" => {
                        let input_line = format!("std::io::stdin().read_line(&mut {}).expect(\"{}\");", splitline[1], splitline[2..].join(" "));
                        let finishedline = input_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    // This trims without conversion. Like I said, someone will be happy when they see it.
                    "inputc" => {
                        let input_line = format!("std::io::stdin().read_line(&mut {}).expect(\"{}\"); {} = {}.trim();", splitline[1], splitline[2..].join(" "), splitline[1], splitline[1]);
                        let finishedline = input_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    "if" => {
                        let condition = splitline[1..].join(" ");
                        let if_line = format!("if {} {{", condition);
                        let finishedline = if_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    "elif" => {
                        let condition = splitline[1..].join(" ");
                        let elif_line = format!("else if {} {{", condition);
                        let finishedline = elif_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    "else" => {
                        let else_line = String::from("else {");
                        codebase.push(else_line);
                        linecounter += 1
                    }

                    // Originally this was around five different closure keywords, but for simplicity it's now a universal word
                    "endblock" => {
                        let endif_line = String::from("}");
                        codebase.push(endif_line);
                        linecounter += 1
                    }

                    // 1 is variable name
                    // 2 is value
                    "assign" => {
                        let assign_line = format!("{} = {};", splitline[1], splitline[2..].join(" "));
                        let finishedline = assign_line;
                        codebase.push(finishedline);
                        linecounter += 1
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
                            linecounter += 1
                        }
                        else {
                            let func_line = format!("fn {}() {{", splitline[1]);
                            let finishedline = func_line;
                            codebase.push(finishedline);
                            linecounter += 1
                        }
                    }

                    // while true (lol)
                    "while" => {
                        let condition = splitline[1..].join(" ");
                        let while_line = format!("while {} {{", condition);
                        let finishedline = while_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    // this is also syntactic sugar/QoL
                    // it helps with making a counted loop with simple syntax
                    // as opposed to for looping and setting up other things
                    "repeat" => {
                        let times = splitline[1];
                        let repeat_line = format!("for _ in 0..{} {{", times);
                        let finishedline = repeat_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    "loop" => {
                        let loop_line = String::from("loop {");
                        codebase.push(loop_line);
                        linecounter += 1
                    }

                    "for" => {
                        let iterator = splitline[1];
                        let range_start = splitline[3..].join(" ");
                        let for_line = format!("for {} in {} {{", iterator, range_start);
                        let finishedline = for_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    "closure" => {
                        if splitline.len() >= 3 {
                            let closure_name = splitline[1];
                            let closure_params = splitline[2..].join(", ");
                            let closure_line = format!("let {} = |{}| {{", closure_name, closure_params);
                            let finishedline = closure_line;
                            codebase.push(finishedline);
                            linecounter += 1
                        }
                        else {
                            let closure_name = splitline[1];
                            let closure_line = format!("let {} = || {{", closure_name);
                            let finishedline = closure_line;
                            codebase.push(finishedline);
                            linecounter += 1
                        }
                    }

                    // This here can call both functions and closures, it's pretty nice
                    "call" => {
                        if splitline.len() >= 3 {
                            let func_name = splitline[1];
                            let func_args = splitline[2..].join(", ");
                            let call_line = format!("{}({});", func_name, func_args);
                            let finishedline = call_line;
                            codebase.push(finishedline);
                            linecounter += 1
                        }
                        else {
                            let func_name = splitline[1];
                            let call_line = format!("{}();", func_name);
                            let finishedline = call_line;
                            codebase.push(finishedline);
                            linecounter += 1
                        }
                    }

                    // Inline Rust
                    "rustc" => {
                        let rustc_line = splitline[1..].join(" ");
                        codebase.push(rustc_line);
                        linecounter += 1
                    }

                    // Inline Assembly
                    "asm" => {
                        let asm_line = format!("unsafe {{ core::arch::asm!({}); }}", splitline[1..].join(" "));
                        let finishedline = asm_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    // qdef = quick define
                    // allows for quick definitions of common types of variables
                    // this is less about necessity, and more QoL/syntactic sugar for the language
                    "qdef" => {
                        match splitline[1] {
                            // nstr here stands for new string
                            "nstr" => {
                                let qdef_line = format!("let mut {} = String::new();", splitline[2]);
                                let finishedline = qdef_line;
                                codebase.push(finishedline);
                            }

                            // same for nvec, new vector
                            "nvec" => {
                                let qdef_line = format!("let mut {} = vec![{}];", splitline[2], splitline[3..].join(", "));
                                let finishedline = qdef_line;
                                codebase.push(finishedline);
                            }

                            // ostr stands for owned string, here the string allows for actual value
                            "ostr" => {
                                let qdef_line = format!("let mut {} = String::from(\"{}\");", splitline[2], splitline[3..].join(" "));
                                let finishedline = qdef_line;
                                codebase.push(finishedline);
                            }

                            // nmap stands for new map
                            "nmap" => {
                                let qdef_line = format!("let mut {}: std::collections::HashMap<_, _> = std::collections::HashMap::new();", splitline[2]);
                                let finishedline = qdef_line;
                                codebase.push(finishedline);
                            }

                            _ => {
                                panic!("Unknown qdef type: {}", splitline[1]);
                            }
                        }
                        linecounter += 1
                    }

                    "qfunc" => {
                        match splitline[1] {
                            "addvar" => {
                                let qfunc_line = format!("{} += {};", splitline[2], splitline[3]);
                                let finishedline = qfunc_line;
                                codebase.push(finishedline);
                            }

                            "subvar" => {
                                let qfunc_line = format!("{} -= {};", splitline[2], splitline[3]);
                                let finishedline = qfunc_line;
                                codebase.push(finishedline);
                            }

                            "mulvar" => {
                                let qfunc_line = format!("{} *= {};", splitline[2], splitline[3]);
                                let finishedline = qfunc_line;
                                codebase.push(finishedline);
                            }

                            "divvar" => {
                                let qfunc_line = format!("{} /= {};", splitline[2], splitline[3]);
                                let finishedline = qfunc_line;
                                codebase.push(finishedline);
                            }

                            "pushvec" => {
                                let qfunc_line = format!("{}.push({});", splitline[2], splitline[3..].join(" "));
                                let finishedline = qfunc_line;
                                codebase.push(finishedline);
                            }

                            _ => {
                                panic!("Unknown qfunc type: {}", splitline[1]);
                            }
                        }
                        linecounter += 1
                    }

                    "match" => {
                        let match_value = splitline[1..].join(" ");
                        let match_line = format!("match {} {{", match_value);
                        let finishedline = match_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    "case" => {
                        let case_value = splitline[1..].join(" ");
                        let case_line = format!("{} => {{", case_value);
                        let finishedline = case_line;
                        codebase.push(finishedline);
                        linecounter += 1
                    }

                    "//" => {
                        let comment_line = format!("// {}", splitline[1..].join(" "));
                        codebase.push(comment_line);
                        linecounter += 1
                    }

                    "" => {

                    }

                    _ => {
                        linecounter += 1;
                        panic!("Unknown command: {} at line {}", splitline[0], linecounter);
                    }
                }
            }
            None => {}
        }
    }
    // After all of the lines are processed, we put them into a single fn main() wrapper.
    // This is done because Aquamarine is more like Python rather than Rust despite being low level, so all logic works in main as if there were no other functions.
    // In essence, it keeps the structure of Aquamarine.
    let transcode = codebase.join("\n");
    let final_code = format!("fn main() {{\n{}\n}}", transcode);
    fs::write(endpoint, final_code).expect("Should have been able to write the file");
    // This allows for multiple flags to be used, mainly for different file outputs depending on user wants.
    if flag == "--asm" {
        let _rustcabuse = Command::new("rustc").arg("--emit=asm").arg(endpoint).output().expect("Failed to compile the code");
    }
    else if flag == "--rustc" {}
    else {
    let _rustcabuse = Command::new("rustc").arg(endpoint).spawn().expect("Failed to compile the code").wait().expect("Failed to wait on child process");
    }
}

