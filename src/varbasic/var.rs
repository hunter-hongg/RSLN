#![allow(unused_variables)]
use std::collections::HashMap;

pub fn var(args: &[&str], var: &mut HashMap<String,i32>){
    if args.len() < 2 {
        println!("\x1b[01;31m参数不足: var\x1b[0m\x1b[1m");
        return;
    } else if args.len() > 2 {
        println!("\x1b[01;33m警告: 过多参数: var, 自动舍弃后续参数\x1b[0m\x1b[1m");
    }
    let realvar: i32 = match args[1]
        .trim()
        .parse::<i32>() {
            Ok(v) => v,
            Err(e) => {
                println!("\x1b[01;31m第二个参数非整数: var\x1b[0m\x1b[1m");
                return;
            },
        };
    match var.insert(args[0].to_string(), realvar){
        Some(x) => println!("\x1b[01;34m覆盖成功, 覆盖前变量为{}\x1b[0m\x1b[1m", x), 
        None => println!("\x1b[01;34m新建成功\x1b[0m\x1b[1m"), 
    };
}
