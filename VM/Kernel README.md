# Kernel

The VM has a kernel, unlike the hardware where everything is run on ring 0. This kernel is written in Rust and handles first boot login and error codes. It also manages the userspace, written in Python.

## File Path Checker

```
    // Kernel control start
    let file_path = "Userspace\\Transfer Pipelines\\Credentials\\username.json";
    let file_path2 = "Userspace\\Transfer Pipelines\\Credentials\\password.json";
    let file_path3 = "Userspace\\Transfer Pipelines\\unsafe_mode.json";
    if Path::new(file_path).exists() && Path::new(file_path2).exists() && Path::new(file_path3).exists() {
        userspace();
    }
```

This part of the code handles config files. username and password are stored in Credentials folder, plaintext (will be changed dw), and if all files exist it will launch userspace, no questions asked. This avoids the setup process below. 

## Setup

```
    else {
        // Answer setup
        let mut username = String::new();
        let mut password = String::new();
        let mut answer = String::new();

        // Intro
        println!("It seems to be your first time here!");
        println!("Let's get you acquainted!");
        println!("First things first, username and password!");

        // Username
        println!("What would you like your username to be?");
        io::stdin().read_line(&mut username).expect("Failed to read username value!");
        let init = fs::write(file_path, username.trim());

        // Password
        println!("Next, what would you like your password to be?");
        io::stdin().read_line(&mut password).expect("Failed to read username value!");
        let init2 = fs::write(file_path2, password.trim());

        // Unsafe
        println!("Now, would you like unsafe mode on your OS?");
        println!("This will give you more control, but you'll likely get more unrecoverable errors.");
        println!("Would you? (1 = yes/0 = no)");
        io::stdin().read_line(&mut answer).expect("Failed to read answer!");
        let init3 = fs::write(file_path3, answer.trim());
```

If first boot was not initialized with all config files, setup will be initialized. The sections of the intro are labelled in the above code block.

## Setup Error Checker

```
        match init {
            Ok(_value) => {
                match init2 {
                    Ok(_value) => {
                        match init3 {
                            Ok(_value) => {
                                println!("Initialized {}", file_path);
                                println!("Initialized {}", file_path2);
                                println!("Initialized {}", file_path3);
                                userspace();
                            }

                            Err(_value) => {
                                println!("Catastrophic Failure, Unsafe Mode Read")
                            }
                        }
                    }

                    Err(_value) => {
                        println!("Catastrophic Error, Password Read")
                    }
                }
            }

            Err(_value) => {
                println!("Catastrophic Error, Username Read");
            }
        }
    }
```

This is the error checker to make sure the files exist exactly where they need to be. If they aren't, it will throw an error.

## Userspace

```
fn userspace() {
    loop {
        // The actual userspace
        let mut binding = Command::new("py");
        let cmd = binding.arg("Userspace\\OS.py");
        let mut passover = cmd.spawn().expect("Failure!");
        passover.wait().expect("Userspace transfer failed.");

        // Logoff and error handler
        let statuscode = fs::read_to_string("Userspace\\Transfer Pipelines\\statuscode.stat").expect("lol").parse::<i32>().unwrap(); // Reads the status code from the statuscode channel
        statuscheck(statuscode)
    }
}
```

The userspace function.

This is put in a loop with statuscheck() such that any new call will go back to the userspace without adding a stack frame, preventing slowdown. The first four lines process and summon a child process, which would be the userspace in the form of OS.py, and use .wait() so the kernel continues to own userspace and Windows does not adopt the py script as an orphan. 

The statuscode.stat file is a file that returns a 32bit integer, which the kernel takes as input for statuschecking. Status 0 means a logoff, in which the logoff handler will execute, any known number triggers an error, and mangled data panics with unrecoverable error thrown.

Example of status check block:

```
        0 => { // Successful error code, meaning a log off
            println!("Are you sure you want to log off? (y/n)");
            let mut tempstring1 = String::new();
            io::stdin().read_line(&mut tempstring1).expect("Failed to read!");
            if tempstring1.trim() == "y" {
                println!("OS Exited.");
                kernelexit();
            }
            else if tempstring1.trim() == "n" {
                userspace();
            }
            else {
                println!("Line read failed!");
                userspace();
            }
        }
```