// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.



struct Foo {
    a: u128,
    b: Option<String>,
}

unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: 调用者必须保证：
    // 1. `ptr` 指向有效的、已分配的 `Foo` 类型内存
    // 2. 该内存最初由 `Box::into_raw` 创建
    // 3. 该指针具有独占所有权（无其他引用）
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    
    // 修改字段值（测试要求）
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: 
        // 1. `Box::into_raw` 返回有效指针
        // 2. 指针来自独占所有的 Box
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert_eq!(ptr_1, ptr_2);          // 验证内存地址不变
        assert_eq!(ret.b, Some("hello".to_owned()));  // 验证字段修改
    }
}