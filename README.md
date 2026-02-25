# Introduction

Welcome to BaushaOS, a small project aimed at getting me to learn more about programming philosophies and problem solving. It includes a hardware userspace for the OS built on MicroPython, a VM (not very similar to hardware) for the OS, driver (hardware) and app support, a markov chain bot, and a custom compiler. The end goal is to either get the compiler onto LLVM or get the OS on the Linux Kernel. (Note: This has been fulfilled with the creation of BaushaOS Linux!)

This doc is primarily written by AI and reviewed by me.

# About

Varanslash (c) 2026

## Hardware

The hardware implementation of BaushaOS runs on a Raspberry Pi Pico 2W using MicroPython firmware. Instead of splitting responsibilities across multiple files or deeply layered modules, the entire system lives inside a single monolithic script. This is intentional. The design favors visibility and structural clarity over physical separation. Everything exists in one place, but it is divided into clearly defined sections that behave like subsystems.

At boot, the system initializes a set of core state containers:

* `filesystem`
* `apps`
* `drivers`

These are plain Python dictionaries. There is no external storage driver, no block device abstraction. Persistence is simulated through a single file called `config.bau`, which stores serialized JSON representations of the filesystem, apps, and drivers. On startup, the system attempts to read this file and reconstruct internal state. On shutdown or crash recovery, it writes back to this file.

This means the “disk” of BaushaOS is effectively a structured snapshot of dictionaries. It is not pretending to be a real filesystem. It is modeling one.

### Boot Process

The boot sequence is simple but meaningful:

1. Initialize global dictionaries.
2. Attempt to load `config.bau`.
3. Load drivers dynamically using `exec`.
4. Enter the main system loop.

Driver loading is particularly important. Each driver is stored as a list of strings. At boot, those strings are joined and executed. This means drivers are effectively runtime-injected code. There is no sandbox. If a driver crashes, the system triggers the BSOD routine, which saves state and resets the board.

This is raw. It is intentionally raw. It mirrors kernel module loading in spirit, but implemented with MicroPython primitives.

### Persistence Model

Persistence is handled manually:

* On `poweroff`, the system writes state to `config.bau`.
* On a fatal driver or app error, the BSOD routine saves state and calls `machine.reset()`.

There is no journaling, no partial writes, no transactional safety. The philosophy here is durability through simplicity rather than durability through complexity.

Because everything is serialized as JSON, the entire OS state is human-readable. This reinforces the project’s educational focus.

### Error Handling

Error codes are centralized in a list. Each error corresponds to a predefined message. Instead of raising exceptions everywhere, commands call `errorcode(code)` and optionally pause.

Fatal failures trigger `BSOD()`, which:

* Forces garbage collection
* Saves system state
* Hard resets the device

This simulates kernel panic behavior in a minimal way.

### Shell and System Loop

The main loop operates as a command interpreter:

```
while True:
    cmdlet = input("BaushaOS> ").split()
```

This is the core of the hardware userspace. Every subsystem hangs off this loop.

Commands include:

* echo
* python (embedded MicroPython REPL)
* dir
* txtedit
* sapphire (driver creation and editing)
* balengu (app creation and editing)
* apprun
* aquamarine (language compilation)
* vortex (Markov bot)
* random
* poweroff

Each command is implemented directly inside the loop. There is no command registry or dispatch table. The structure is explicit and linear.

This keeps the system readable at the cost of scalability. That tradeoff is aligned with the project’s goals.

### Filesystem Model

The filesystem is a dictionary:

```
filesystem = {
    "filename": ["line1", "line2", ...]
}
```

Files are stored as lists of lines. Directories are not hierarchical. There is no nested structure. The `dir` command manipulates this dictionary directly.

This is not meant to replicate a Unix filesystem. It is a controlled model for experimenting with storage abstraction.

Text editing is implemented through `txtedit`, which appends lines to a file’s list. There is no cursor model, no random access editing. It is append-based.

### Apps and Drivers

Apps and drivers follow the same pattern:

```
apps = {
    "appname": ["line1", "line2", ...]
}

drivers = {
    "drivername": ["line1", "line2", ...]
}
```

Both are stored as executable string lists.

Apps are executed on demand using:

```
exec("\n".join(apps[name]))
```

Drivers are executed automatically at boot.

The difference between an app and a driver is behavioral, not structural. This reflects the project’s architectural unification philosophy.

### MicroAqua / Aquamarine Interpreter

The MicroAqua interpreter is a lightweight transpiler that converts custom language lines into Python code.

It works by:

* Maintaining a `collectlines` buffer.
* Tracking `indent`.
* Translating commands into Python syntax.
* Appending formatted Python strings into the buffer.

Indentation is manually managed through:

* `loop`, `for`, `while`, `if`, `fn` increasing indent.
* `endblock` decreasing indent.
* `rblock` resetting indent to zero.
* `iblock` manually increasing indent.

There is no AST. There is no parser tree. It is direct string transformation. The language is structural, not semantic-heavy.

Once translation is complete, the resulting Python code is stored as a new app entry.

This means Aquamarine is not executed directly. It compiles into MicroPython code, which then executes under the same runtime model as any other app.

### Vortex Markov Chain Bot

The Vortex bot maintains a global dictionary called `words`, mapping each word to a list of possible successor words.

Training works by reading text and building adjacency lists.

Generation works by:

* Starting from a seed word.
* Randomly selecting the next word from the adjacency list.
* Repeating for a given length.

The bot is fully integrated into the shell via the `vortex` command.

This demonstrates probabilistic computation inside the OS environment without external libraries.

### Embedded Python Mode

The `python` command opens a raw MicroPython REPL inside the OS.

This effectively allows escape from the structured shell into direct execution.

This dual-layer interaction model is significant:

* Structured commands for OS-level operations.
* Direct Python for experimentation and debugging.

# VM

## Kernel

The Kernel is written in Rust and acts as the supervisory control layer of the virtual machine. It is responsible for:

### First time initialization
On startup, the Kernel checks for the existence of three credential files:

Userspace\Transfer Pipelines\Credentials\username.json
Userspace\Transfer Pipelines\Credentials\password.json
Userspace\Transfer Pipelines\unsafe_mode.json

If any of these do not exist, the Kernel:

Prompts the user for a username and password

Asks whether unsafe mode should be enabled

Writes these values to their respective files

Transfers control to Userspace upon successful initialization

### Userspace execution loop
After initialization, the Kernel enters an infinite loop where it:

Spawns the Python OS process (Userspace\OS.py)

Waits for the process to terminate

Reads a status code from Userspace\Transfer Pipelines\statuscode.stat

Matches on that status code to determine behavior

### Status code handling
Status codes are treated as a granular control channel between Userspace and Kernel.

#### Code 0
Indicates a logoff request.
The Kernel prompts the user for confirmation.
If confirmed, the Kernel exits. Otherwise, it restarts Userspace.

#### Code 1
Indicates too many failed login attempts.
The Kernel exits immediately.

#### Any other code
Treated as a catastrophic error.
A BSOD style message is printed and the loop continues after a short delay.

### Kernel exit
kernelexit() sleeps briefly and then terminates the process cleanly.

Architecturally, this Kernel functions as:

Process supervisor

Authentication initializer

Error boundary

Status channel interpreter

It does not implement OS logic. It delegates that to Userspace.

## Userspace

### OS

The OS is written in Python and operates as the interactive environment for the user. It runs as a subprocess controlled by the Kernel.

#### Authentication
On startup, the OS:

Reads username and password from credential files

Prompts the user to enter credentials in the format: username, password

Allows three login attempts

On failure after three attempts, returns status code 1 via mof.statusreturn(1)

On success, enters the command loop

#### Environment initialization
After successful login:

Displays an ASCII banner

Displays copyright information

Retrieves hostname via socket.gethostname()

Enters a command interpreter loop

#### Command interpreter

Supported commands:

run <app>
Executes Userspace\Apps<app>.py via PowerShell.

logoff
Sends status code 0 back to Kernel using mof.statusreturn("0").

echo <string>
Prints the provided string.

eval <expression>
Evaluates a Python expression using eval() and prints the result.

powershell <cmd>
Executes arbitrary PowerShell commands.

dir create file <name>
Creates a file in Userspace\Filesystem.

dir create dir <name>
Creates a directory in Userspace\Filesystem.

dir delete <name>
Removes a file or directory from Userspace\Filesystem.

sleep <sec>
Sleeps for the specified number of seconds.

#### Control model
The OS does not exit directly.
Instead, it communicates termination or error states via statusreturn(), which writes to the status channel file.
The Kernel then decides what to do.

## Status Channels

Status Channels are file based IPC mechanisms between Kernel and Userspace.

Primary channel:
Userspace\Transfer Pipelines\statuscode.stat

Mechanism:

Userspace writes an integer status code to this file.

Kernel reads the file after Userspace process termination.

Kernel matches on the integer and determines the next control action.

This creates a simple supervisory model:

Kernel
↕ file based status channel
Userspace

Aquamarine Transpiler Documentation
Overview

The Aquamarine Transpiler is a custom Rust-based transpiler designed to convert your Aquamarine language scripts into valid Rust code. Its purpose is to make Aquamarine resemble a high-level scripting language (Python/Pseudocode style) while still leveraging Rust’s compilation, speed, and low-level capabilities.

The transpiler supports inline Rust and assembly, control flow, variable manipulation, functions, closures, loops, and other syntactic sugar. Its design favors flexibility and ease-of-use, even allowing potentially unsafe operations, reflecting its experimental nature.

Features

Converts Aquamarine commands to valid Rust syntax.

Supports control flow constructs: if, elif, else, while, for, loop, repeat, match, case.

Handles variable declaration and assignment, including mutable (var mut) and immutable (var !mut) variables.

Allows inline Rust (rustc) and inline assembly (asm) embedding.

Offers quick variable/function shorthand:

qdef for rapid declaration of strings, vectors, and maps.

qfunc for operations on variables and vectors (addvar, subvar, pushvec, etc.).

Supports functions and closures, both definition (func, closure) and invocation (call).

Preserves comments and supports flexible whitespace handling.

Compiles resulting Rust code either to:

Standard executable (via rustc)

Assembly output (--asm flag)

Installation

Make sure Rust is installed on your system.

Place the transpiler file (e.g., aquamarine_transpiler.rs) in a directory of your choice.

Compile the transpiler:

rustc aquamarine_transpiler.rs -o aquamarine_transpiler

Run it with:

./aquamarine_transpiler <input_file.aqua> <output_file.rs> [--asm|--rustc]

<input_file.aqua> — the source file written in Aquamarine

<output_file.rs> — the Rust output file

Optional flags:

--asm or -a: generate assembly instead of compiling a binary

--rustc or -r: leave Rust code for manual compilation

Command Mapping
Aquamarine Command	Rust Translation	Notes
echo <text>	println!(<text>);	Prints text to stdout
var mut <name> <value>	let mut <name> = <value>;	Mutable variable
var !mut <name> <value>	let <name> = <value>;	Immutable variable
assign <var> <value>	<var> = <value>;	Variable reassignment
input <var> <message>	Reads input, trims, converts to string	Ensures .trim().to_string()
inputn <var> <message>	Reads input with newline preserved	No conversion
inputc <var> <message>	Reads input, trims, no conversion	.trim() only
sleep <ms>	std::thread::sleep(std::time::Duration::from_millis(<ms>));	Milliseconds-based sleep
import <crate>	use <crate>;	Standard Rust imports
func <name> [params]	fn <name>(params) {	Function definition
closure <name> [params]	`let <name> =	params
call <name> [args]	<name>(args);	Call functions or closures
if <condition>	if <condition> {	Conditional start
elif <condition>	else if <condition> {	Else-if start
else	else {	Else start
endblock	}	Ends a block
while <condition>	while <condition> {	Loops
loop	loop {	Infinite loop
repeat <times>	for _ in 0..<times> {	Counted loop
for <var> in <range>	for <var> in <range> {	Iterative loop
match <value>	match <value> {	Match start
case <value>	<value> => {	Case inside match
rustc <line>	Inline Rust	Arbitrary Rust code
asm <line>	Inline assembly	Unsafe block, uses core::arch::asm!
qdef nstr <name>	let mut <name> = String::new();	Quick define string
qdef ostr <name> <value>	let mut <name> = String::from(<value>);	Quick define owned string
qdef nvec <name> [values...]	let mut <name> = vec![values];	Quick define vector
qdef nmap <name>	let mut <name>: HashMap<_, _> = HashMap::new();	Quick define map
qfunc <operation> <target> <arg>	Rust equivalent (+=, -=, push, etc.)	Quick function shorthand
Flags

--asm / -a: compile the resulting Rust file to assembly output instead of an executable.

--rustc / -r: generate Rust code for manual compilation later (no immediate compilation).

Default behavior: compile Rust code immediately using rustc.

Usage Example

Aquamarine source (hello.aqua):

var mut name "Varan"
echo "Hello, {{}}" name
sleep 1000

Running transpiler:

./aquamarine_transpiler hello.aqua hello.rs

Generated Rust (hello.rs):

fn main() {
    let mut name = "Varan";
    println!("Hello, {}", name);
    std::thread::sleep(std::time::Duration::from_millis(1000));
}

Compile:

rustc hello.rs
./hello

Output:

Hello, Varan
Security Notes

This transpiler executes arbitrary Rust and assembly code.

Any Aquamarine script can inject unsafe operations, system calls, or memory manipulation.

Use only with trusted code. Running untrusted scripts can compromise your system.

Design Philosophy

Aquamarine is high-level yet close to Rust, aiming for beginner readability and experimental low-level features.

It preserves Python-like simplicity (linear code structure) while leveraging Rust’s type system, memory safety, and concurrency when compiled.

It allows advanced users to embed Rust and assembly freely, bridging the gap between scripting and low-level control.

Flags and quick shorthand functions (qdef, qfunc) provide developer ergonomics for rapid prototyping.
