use std::env;
use std::fs;
use std::process::Command;

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

        else if splitline[0] == "sleep" {
            let sleep_line = format!("std::thread::sleep(std::time::Duration::from_millis({}));", splitline[1]);
            let finishedline = sleep_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "import" {
            let import_line = format!("use {};", splitline[1]);
            let finishedline = import_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "input" {
            let input_line = format!("std::io::stdin().read_line(&mut {}).expect(\"{}\");", splitline[1], splitline[2..].join(" "));
            let finishedline = input_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "if" {
            let condition = splitline[1..].join(" ");
            let if_line = format!("if {} {{", condition);
            let finishedline = if_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "endif" {
            let endif_line = String::from("}");
            codebase.push(endif_line);
        }

        else if splitline[0] == "assign" {
            let assign_line = format!("{} = {};", splitline[1], splitline[2]);
            let finishedline = assign_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "func" {
            let func_line = format!("fn {}() {{", splitline[1]);
            let finishedline = func_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "endfunc" {
            let endfunc_line = String::from("}");
            codebase.push(endfunc_line);
        }

        else if splitline[0] == "while" {
            let condition = splitline[1..].join(" ");
            let while_line = format!("while {} {{", condition);
            let finishedline = while_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "endwhile" {
            let endwhile_line = String::from("}");
            codebase.push(endwhile_line);
        }

        else if splitline[0] == "repeat" {
            let times = splitline[1];
            let repeat_line = format!("for _ in 0..{} {{", times);
            let finishedline = repeat_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "endrepeat" {
            let endrepeat_line = String::from("}");
            codebase.push(endrepeat_line);
        }

        else if splitline[0] == "loop" {
            let loop_line = String::from("loop {");
            codebase.push(loop_line);
        }

        else if splitline[0] == "endloop" {
            let endloop_line = String::from("}");
            codebase.push(endloop_line);
        }

        else if splitline[0] == "for" {
            let iterator = splitline[1];
            let range_start = splitline[3];
            let range_end = splitline[5];
            let for_line = format!("for {} in {}..{} {{", iterator, range_start, range_end);
            let finishedline = for_line;
            codebase.push(finishedline);
        }

        else if splitline[0] == "endfor" {
            let endfor_line = String::from("}");
            codebase.push(endfor_line);
        }

        else if splitline[0] == "//" {}

        else if splitline[0] == "" {}
    let transcode = codebase.join("\n");
    let final_code = format!("fn main() {{\n{}\n}}", transcode);
    fs::write(endpoint, final_code).expect("Should have been able to write the file");
    }

    let rustcabuse = Command::new("rustc")
        .arg(endpoint)
        .output()
        .expect("Failed to compile the code");
}