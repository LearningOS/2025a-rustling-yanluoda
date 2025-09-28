// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 使用 if let 代替 for 循环
    if let Some(x) = option {
        res += x;
    }
    
    println!("{}", res);
}