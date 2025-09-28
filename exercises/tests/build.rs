//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 1. 设置 TEST_FOO 环境变量
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);  // ✅ 设置环境变量[7](@ref)

    // 2. 启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");  // ✅ 启用特性[9](@ref)
}