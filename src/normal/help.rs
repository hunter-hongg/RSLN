use super::super::name;

pub fn help() {
    print!("\x1b[34m");
    println!("{}是一个命令行工具", name::getname());
    println!("它支持命令如下: ");
    println!("----------    基础命令    ----------");
    println!("[c|clear] -- 清空屏幕");
    println!("[e|exit] -- 退出本工具");
    println!("[h|help] -- 打印本帮助");
    println!("[v|ver|version] -- 输出版本");
    println!("----------    变量储存    ----------");
    println!("<varname1> [<varname2>...] -- 读取任意个变量的值，要求第一个变量存在且不为命令");
    println!("[cp|copy] <var> <new> -- 将<var>的值复制给后一个变量");
    println!("[g|get] <varname1> [<varname2>...] -- 读取任意个变量的值");
    println!("[_|var] <varname> <value> -- 新建或覆盖一个变量");
    println!("[w|write] -- 格式化输出所有变量");
    println!("----------    变量操作    ----------");
    println!("add <var> <add> -- 为一个变量增加指定的值");
    println!("rsub <sub> <var> -- 用一个指定的值减变量");
    println!("sub <var> <sub> -- 为一个变量减少指定的值");
    println!("mul <var> <mul> -- 将一个数与一个变量相乘");
    println!("div <var> <div> -- 将一个变量除以一个数");
    println!("rdiv <div> <var> -- 用一个数除以一个变量");
    println!("----------    数字运算    ----------");
    println!("[bo|bitops] [!|~|^] [<var>|<num>] -- 按位取反，传入数字结果存于answer，传入变量结果覆盖原变量");
    println!("[bo|bitops] [<var1>|<num1>] & [<var2>|<num2>] -- 按位与，结果存于answer");
    println!("[bo|bitops] [<var1>|<num1>] | [<var2>|<num2>] -- 按位或，结果存于answer");
    println!("[bo|bitops] [<var1>|<num1>] ^ [<var2>|<num2>] -- 按位异或，结果存于answer");
    println!("[bo|bitops] [<var1>|<num1>] << [<var2>|<num2>] -- 按位左移，结果存于answer");
    println!("[bo|bitops] [<var1>|<num1>] >> [<var2>|<num2>] -- 按位右移，结果存于answer");
    print!("\x1b[0m\x1b[1m");
}
