use std::collections::HashMap;

pub fn rsubvar(args: &[&str], var: &mut HashMap<String,i32>){
    let mut Variable = var;
    if args.len() < 2 {
        println!("\x1b[01;31m参数不足: sub\x1b[0m\x1b[1m");
        return;
    } else if args.len() > 2 {
        println!("\x1b[01;33m警告: 过多参数: rsub, 自动舍弃后续参数\x1b[0m\x1b[1m");
    }
    let subnum = match args[0]
        .trim()
        .parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                println!("\x1b[01;31m参数2非整数: sub\x1b[0m\x1b[1m");
                return;
            }, 
        };
    let before = match Variable.get(&args[1].to_string()) {
        Some(x) => x,
        None => {
            println!("\x1b[01;31m未找到{}变量: rsub\x1b[0m\x1b[1m", args[0]);
            return;
        },
    };
    let _ = Variable.insert(args[1].to_string(), subnum - before);
    println!("\x1b[01;34m减少成功\x1b[0m\x1b[1m");
}
