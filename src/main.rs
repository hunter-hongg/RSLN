#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
// ANSI转义使用\x1b而非\033

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use RSLN::*;

fn main() {
    let mut VarInt: HashMap<String, i32> = HashMap::new();
    print!("\x1b[1m");
    version::putver();
    println!();
    'a: loop {
        print!("\x1b[01;35m{} > \x1b[37m", name::getname());
        let _ = io::stdout().flush();
        let mut all_command : String = String::new();
        io::stdin()
            .read_line(&mut all_command)
            .expect("Failed to read stdin");
        let commands_all: Vec<&str> = all_command
            .as_str()
            .split_whitespace()
            .collect();
        if commands_all.len() == 0 {
            println!();
            continue 'a;
        }
        let command = commands_all[0];
        let args = &commands_all[1..];
        print!("\x1b[0m\x1b[1m");
        match command {
            "h" | "help" => normal::help(),
            "c" | "clear" => normal::clear(), 
            "e" | "exit" => break 'a,
            "_" | "var" => varbasic::var(args, &mut VarInt),
            "g" | "get" => varbasic::getvar(args, &VarInt),
            "add" => opvar::addvar(args, &mut VarInt),
            "sub" => opvar::subvar(args, &mut VarInt),
            "rsub" => opvar::rsubvar(args, &mut VarInt),
            "mul" => opvar::mulvar(args, &mut VarInt), 
            "div" => opvar::divvar(args, &mut VarInt),
            "rdiv" => opvar::rdivvar(args, &mut VarInt), 
            "cp" | "copy" => opvar::cpvar(args, &mut VarInt),
            "bo" | "bitops" => op::bovar(args, &mut VarInt),
            "w" | "write" => normal::write(&VarInt), 
            "v" | "ver" | "version" => version::putver(), 
            _ => varbasic::nget(&commands_all[..], &VarInt), 
        } // _=var, c=clear, e=exit, h=help, v=version, w=write, 
    }
}
