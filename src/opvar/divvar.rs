use std::collections::HashMap;

pub fn divvar(args: &[&str], var: &mut HashMap<String,i32>){
    let Variable = var;
    if args.len() < 2 {
        println!("\x1b[01;31m参数不足: div\x1b[0m\x1b[1m");
        return;
    } else if args.len() > 2 {
        println!("\x1b[01;33m警告: 过多参数: div, 自动舍弃后续参数\x1b[0m\x1b[1m");
    }
    let divnum = match args[1]
        .trim()
        .parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                println!("\x1b[01;31m参数2非整数: div\x1b[0m\x1b[1m");
                return;
            }, 
        };
    if divnum == 0 {
        println!("\x1b[01;31m除以0: div\x1b[0m\x1b[1m");
        return;
    }
    let before = match Variable.get(&args[0].to_string()) {
        Some(x) => x,
        None => {
            println!("\x1b[01;31m未找到{}变量: div\x1b[0m\x1b[1m", args[0]);
            return;
        },
    };
    let _ = Variable.insert(args[0].to_string(), before / divnum);
    println!("\x1b[01;34m相除成功\x1b[0m\x1b[1m");
}
pub fn rdivvar(args: &[&str], var: &mut HashMap<String,i32>){
    let Variable = var;
    if args.len() < 2 {
        println!("\x1b[01;31m参数不足: rdiv\x1b[0m\x1b[1m");
        return;
    } else if args.len() > 2 {
        println!("\x1b[01;33m警告: 过多参数: rdiv, 自动舍弃后续参数\x1b[0m\x1b[1m");
    }
    let divnum = match args[0]
        .trim()
        .parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                println!("\x1b[01;31m参数1非整数: rdiv\x1b[0m\x1b[1m");
                return;
            }, 
        };
    let before = match Variable.get(&args[1].to_string()) {
        Some(x) => x,
        None => {
            println!("\x1b[01;31m未找到{}变量: rdiv\x1b[0m\x1b[1m", args[1]);
            return;
        },
    };
    if *before == 0 {
        println!("\x1b[01;31m除以0: rdiv\x1b[0m\x1b[1m");
        return;
    }
    let _ = Variable.insert(args[1].to_string(), divnum / before );
    println!("\x1b[01;34m相除成功\x1b[0m\x1b[1m");
}
