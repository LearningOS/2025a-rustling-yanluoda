// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 修复1: 移除无效的unwrap调用[2,6](@ref)
    if my_option.is_none() {
        // 直接移除这行无效代码
    }

    // 修复2: 修复数组语法错误[7,11](@ref)
    let my_arr = &[-1, -2, -3, -4, -5, -6]; // 添加缺失的逗号

    // 修复3: 正确创建空向量[1,5](@ref)
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear(); // 使用clear()替代resize(0)
    let my_empty_vec = my_vec;

    // 修复4: 正确交换变量值[7,11](@ref)
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b); // 使用内存交换函数

    println!("My array! Here it is: {:?}", my_arr);
    println!("This Vec is empty, see? {:?}", my_empty_vec);
    println!("value a: {}; value b: {}", value_a, value_b);
}