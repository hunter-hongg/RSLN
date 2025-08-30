use std::collections::HashMap;

pub fn addvar(args: &[&str], var: &mut HashMap<String,i32>){
    if args.len() < 2 {
        println!("\x1b[01;31m参数不足: add\x1b[0m\x1b[1m");
        return;
    } else if args.len() > 2 {
        println!("\x1b[01;33m警告: 过多参数: add, 自动舍弃后续参数\x1b[0m\x1b[1m");
    }
    let realadd: i32 = match args[1]
        .trim()
        .parse::<i32>() {
            Ok(v) => v,
            Err(_e) => {
                println!("\x1b[01;31m第二个参数非整数: add\x1b[0m\x1b[1m");
                return;
            },
        };
    let before = match var.get(&args[0].to_string()){
        Some(x) => x,
        None => {
            println!("\x1b[01;31m变量不存在: {}\x1b[0m\x1b[1m", args[0]);
            return;
        }
    };
    let _ = var.insert(args[0].to_string(), before + realadd);
    println!("\x1b[01;34m增加成功\x1b[0m\x1b[1m");
}
