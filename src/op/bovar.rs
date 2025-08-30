use std::collections::HashMap;

pub fn bovar(args: &[&str], var: &mut HashMap<String,i32>){
    match args.len() {
        2 => {
            match args[1]
                .trim()
                .parse::<i32>() {
                    Ok(x) => {
                        let _ = var.insert(String::from("answer"), !x);
                    }, 
                    Err(_) => {
                        match var.get(&args[1].to_string()) {
                            Some(y) => {
                                let _ = var.insert(args[1].to_string(), !y);
                            },
                            None => println!("\x1b[01;31m{}变量不存在\x1b[0m\x1b[1m", args[1]),
                        }
                    }, 
            };
        }, 
        3 => {
            let first = match args[0]
                .trim()
                .parse::<i32>() {
                    Ok(x) => x, 
                    Err(_) => {
                        match var.get(&args[0].to_string()){
                            Some(y) => *y, 
                            None => {
                                println!("\x1b[01;31m{}变量不存在\x1b[0m\x1b[1m", args[0]);
                                return;
                            },
                        }
                    },
                };
            let second = match args[2]
                .trim()
                .parse::<i32>() {
                    Ok(x) => x, 
                    Err(_) => {
                        match var.get(&args[2].to_string()){
                            Some(y) => *y, 
                            None => {
                                println!("\x1b[01;31m{}变量不存在\x1b[0m\x1b[1m", args[2]);
                                return;
                            },
                        }
                    },
                };
            let ans = match args[1] {
                "&" => first&second, 
                "|" => first|second, 
                "^" => first^second, 
                "<<" => first << second, 
                ">>" => first >> second,
                _ => {
                    println!("\x1b[01;31m操作不存在: {}\x1b[0m\x1b[1m", args[1]);
                    return;
                }, 
            };
            let _ = var.insert(String::from("answer"), ans);
        }, 
        _ => {
            println!("\x1b[01;31m参数错误: bo\x1b[0m\x1b[1m");
            return;
        }, 
    }
} 
