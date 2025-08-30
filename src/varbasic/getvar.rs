use std::collections::HashMap;

pub fn getvar(args: &[&str], var: &HashMap<String,i32>) {
    for i in args {
        match var.get(&i.to_string()) {
            Some(x) => println!("\x1b[01;34m{}的值为{}\x1b[0m\x1b[1m", i, x),
            None => println!("\x1b[01;31m{}变量不存在\x1b[0m\x1b[1m", i),
        }
    }
}
pub fn nget(args: &[&str], var: &HashMap<String,i32>) {
    match var.get(&args[0].to_string()) {
        None => {
            println!("\x1b[01;31m未知命令: {}\x1b[0m\x1b[1m", args[0]);
        }, 
        _ => {
            for i in args {
                match var.get(&i.to_string()) {
                    Some(x) => println!("\x1b[01;34m{}的值为{}\x1b[0m\x1b[1m", i, x),
                    None => println!("\x1b[01;31m{}变量不存在\x1b[0m\x1b[1m", i),
                }
            }
        }, 
    }
}
