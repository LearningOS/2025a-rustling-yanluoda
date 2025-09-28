// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.



/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: 
    // 1. 调用者必须保证 `address` 是有效且对齐的可写内存地址（即指向合法的 `u32` 类型数据）。
    // 2. 调用者必须保证此地址在函数执行期间独占访问（无其他引用或指针同时访问同一内存位置）。
    // 3. 调用者必须保证目标内存的生命周期覆盖此次修改操作。
    unsafe {
        let ptr = address as *mut u32; // 转换地址为裸指针
        *ptr = 0xAABBCCDD;            // 解引用并修改内存值
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: 
        // 1. `&mut t` 生成有效的可变引用，其地址非空且对齐。
        // 2. `t` 是局部变量，生命周期覆盖整个 unsafe 块。
        // 3. 无其他引用同时访问 `t`（满足独占访问）。
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert_eq!(t, 0xAABBCCDD); // 验证内存修改成功
    }
}