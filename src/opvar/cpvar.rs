use std::collections::HashMap;

pub fn cpvar(args: &[&str], var: &mut HashMap<String,i32>){
    if args.len() < 2 {
        println!("\x1b[01;31m参数不足: cp\x1b[0m\x1b[1m");
        return;
    }
    let num = match var.get(&args[0].to_string()) {
        Some(x) => x,
        None => {
            println!("\x1b[01;31m未找到{}变量: cp\x1b[0m\x1b[1m", args[0]);
            return;
        },
    };
    let tmp = *num;
    let i = args[1];
    //for i in &args[1..] {
        println!("\x1b[01;34m成功赋值{}={}\x1b[0m\x1b[1m", i, tmp);
        let _ = var.insert(i.to_string(), *num);
    //}
} 
