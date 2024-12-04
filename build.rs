fn main() {
    // 设置 sqlite3.dll 所在的目录
    println!("cargo:rustc-link-search=native=D:/install/sqlite3");

    // 指定链接动态库 sqlite3
    println!("cargo:rustc-link-lib=dylib=sqlite3");
}
