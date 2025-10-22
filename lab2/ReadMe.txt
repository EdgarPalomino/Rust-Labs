Lab number: CSE 5402 Fall 2025 Lab 1
Zichu Pan p.zichu@wustl.edu
Edgar Palomino e.j.palomino@wustl.edu

Modules
- My solution is organized into 1 module with 2 sub-modules to achieve separation of concerns.
    - `main.rs` is the entry point module that handles
        - command-line argument parsing
        - program control flow cmd-line parsing -> script generation -> script printing
    - `lab1::declarations` is used to define shared constants and types, this increases the readability of the program
        - it also tracks the global `WHINGE_MODE` boolean
    - `lab1::script_gen` contains the main logic of script generation which involves file I/O operations and processing all
      encapsulated in a submodule

Design Challenges
- Both `main.rs` needs to access `WHINGE_MODE` for cmd-line parsing and `lab1::declarations` needs to access for printing.
    - Declared `WHINGE_MODE` as pub static of type AtomicBool, which creates 1 instance that lives for the entire program, with a
      fixed memory address, so every module access the same instance
- Error propagation from script_gen to main
    - All error types are defined in `declarations.rs` meaning main shares the same understanding of error as sub-module

Design Decisions
The most fundamental choice was representing the play as `Vec<(usize, String, String)>` - a flat 
vector of tuples containing line number, character name, and dialogue. This structure enables direct 
sorting via Rust's built-in tuple ordering while avoiding the complexity of nested maps or custom structs. 
The verbose mode implementation uses a global `static AtomicBool`, which results in a dramatically cleaner API instead of passing 
a FLAG to each function. String data is cloned rather than borrowed throughout the program, though not as efficient
as complex lifetime annotations, it reduces the complexity of the code. 

Running the code:
    use cmd to unzip the folder: unzip lab1.zip
    to build the project: cargo build
    Now the program can be run using: target/debug/lab1 <configuration_file_name> [whinge]
    [Note: the config file and part files must be in the root of the directory]

Running Provided Tests:
    IMPORTANT: You need to cd into the directory of test files first. For example to run test_1 cd test/test_1
    CMDS:
        ../../target/debug/lab1 hamlet_ii_2_config.txt whinge
        ../../target/debug/lab1 nerv_config.txt whinge
        ../../target/debug/lab1 seele_config.txt whinge

Testing:
    Test 1: consists of the default test case with the provided hamlet text
    Test 2:
        Config:
            - Contains blank lines that should be skipped
            - Has a line with extra tokens
            - References both valid and empty files
        Parts:
            - `shinji_malformed.txt`: Contains invalid line numbers (text, decimals, negatives), blank lines, and lines with only line numbers
            - `asuka_malformed.txt`: Tests lines without dialogue (line 12), text instead of numbers ("thirteen"), and missing spaces after numbers ("16Synchronization")
            - `gendo_silent.txt`: Contains only unnumbered text
            - `misato_empty.txt`: Contains only whitespace
    Test 3:
        Config:
            - Tests various boundary conditions for line number parsing

        Parts:
            - `kaworu_spaces.txt`: Only whitespace - produces no output
            - `penpen_numbers.txt`: Numbers without dialogue - all lines skipped
            - `kaji_huge.txt`: Tests usize overflow with massive numbers
            - `ritsuko_zero.txt`: Tests line number 0 and leading zeros
    
    The program passed all test cases.
