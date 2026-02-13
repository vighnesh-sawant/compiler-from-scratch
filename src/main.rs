mod ast;
mod codegen;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename.c>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let path = Path::new(input_path);

    let parent_dir = path.parent().unwrap_or(Path::new("."));
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let asm_path = parent_dir.join(format!("{}.s", file_stem));
    let exe_path = parent_dir.join(file_stem);

    println!("Compiling {}...", input_path);

    let tokens = match lexer::lex(input_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer Error: {:?}", e);
            std::process::exit(1);
        }
    };

    let mut parser = parser::Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parser Error: {:?}", e);
            std::process::exit(1);
        }
    };

    let assembly = codegen::generate(&ast);

    if let Err(e) = fs::write(&asm_path, assembly) {
        eprintln!("Failed to write assembly file: {}", e);
        std::process::exit(1);
    }

    println!("Generated assembly: {}", asm_path.display());

    println!("Running GCC...");

    let output = Command::new("gcc")
        .arg(&asm_path)
        .arg("-o")
        .arg(&exe_path)
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("Success! Executable created at: ./{}", exe_path.display());
            } else {
                eprintln!("GCC Error:");
                eprintln!("{}", String::from_utf8_lossy(&result.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute GCC. Is it installed?");
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
