use std::collections::HashMap;

pub fn write(var: &HashMap<String,i32>) {
    print!("\x1b[01;34m");
    println!("{}", "{");
    for (key, value) in var {
        println!("  \"{}\": {}, ", key, value);
    }
    println!("{}", "}");
    print!("\x1b[0m\x1b[1m");
}
