fn main() {
    // 打印环境变量，查看路径设置
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=native=./libs");
    println!("cargo:rustc-link-lib=dylib=sqlite3");

    // 输出一些调试信息
    println!("cargo:warning=Debug:正在链接 sqlite3 库...");
}
