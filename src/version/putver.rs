use super::super::name;

pub fn putver() {
    print!("\x1b[01;37m");
    println!("{} 版本 {}, 请输入你的命令, \"help\" 获取帮助", name::getname(), "3.06");
    print!("\x1b[0m\x1b[1m");
}
