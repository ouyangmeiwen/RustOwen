
struct Body {
    title: Option<String>,
}

fn runtest1(){
    let body: Body = Body {
        title: Some("Hello1".to_string()),
    };
    // 通过 unwrap 获取所有权
    let title: String = body.title.unwrap();
    println!("{}", title);  // 输出: Hello
    //println!("{}", body.title.unwrap());  // 错误: `body.Title` 的所有权已被移动
}
fn runtest2(){
    let body1: Body = Body {
        title: Some("Hello2".to_string()),
    };
    let some_body=body1.title.as_ref().unwrap();
    println!("{}", some_body);  // 输出: Hello
}
fn runtest3(){
    let mut  body2: Body = Body {
        title: Some("Hello3".to_string()),
    };
    // 使用可变引用来修改 Title 字段的值
    if let Some(title2) =  body2.title.as_mut(){
        *title2 = "Hello31".to_string(); // 修改值
        println!("{}", title2);  // 输出: Hello
    }
    println!("{}", body2.title.as_ref().unwrap());  // 输出: Hello
}

fn runtest4(){
    let hello_string = "hello4".to_string(); // 创建一个长生命周期的 String
    let ts = Some(&hello_string); // 使用 hello_string 的引用
    println!("{}", ts.unwrap()); // 现在引用没有问题
}

fn runtest5(){
    let hello_string = "hello5".to_string(); // 创建一个长生命周期的 String
    let ts = Some(hello_string); // 使用 hello_string 的引用
    println!("{}", ts.unwrap()); // 现在引用没有问题
    //println!("{}", hello_string); // 现在引用没有问题 报错
    //println!("{}", ts.unwrap()); // 现在引用没有问题  报错
}
fn runtest6(){
    let hello_string = "hello6".to_string(); // 创建一个长生命周期的 String
    let ts = Some(hello_string); // 使用 hello_string 的引用
    println!("{}", ts.as_ref().unwrap()); // 现在引用没有问题
    //println!("{}", hello_string); // 现在引用没有问题  报错
    println!("{}", ts.as_ref().unwrap()); // 现在引用没有问题
}

fn runmove(){
    let a = String::from("hello"); // a 拥有 String 的所有权
    let b = a; // 所有权转移，从 a 到 b
    println!("{}", b); // 这时可以通过 b 来访问 String
    //println!("{}", a); // 错误！a 已经不再拥有所有权，不能再使用它
}

fn runmove2() {
    let a = String::from("hello");
    //String 类型实现了 AsRef<OsStr>，而不是 AsRef<String>，因此直接调用 as_ref() 返回的是 &OsStr，而不是 &String
    //let c = a.as_ref(); // 这里 `as_ref()` 返回的是 `&String`  报错 未实现 
    //println!("{}", c); // 打印 &String 的内容，`c` 是对 a 的引用
}

fn runcopy(){
    let x = 5; // x 是 i32 类型，类型 i32 实现了 Copy
    let y = x; // 这时发生的是值的复制，而不是所有权转移
    println!("{}", x); // x 仍然有效，可以继续使用
    println!("{}", y); // y 也有效，拥有值 5
}

fn runborrow(){
    let a = String::from("hello"); // a 拥有 String 的所有权
    let b = &a; // b 是对 a 的不可变引用，所有权没有转移
    println!("{}", a); // a 仍然有效
    println!("{}", b); // b 也可以访问 a 的内容
}
fn runclone(){
    let a = String::from("hello"); // a 拥有 String 的所有权
    let b = a.clone(); // b 拥有 a 数据的深拷贝

    println!("{}", a); // a 仍然有效
    println!("{}", b); // b 也可以使用数据
}

fn test1111() {
    let s = String::from("hello");
    let r: &str = &s; // 自动解引用
    println!("{}", r); // 输出 "hello"
    println!("{}", s); // 输出 "hello"
    println!("{}", "hello"); // 输出 "hello"
}



pub  fn  runtest() {
    runtest1();
    runtest2();
    runtest3();
    runtest4();
    runtest5();
    runtest6();
    runmove();
    runcopy();
    runborrow();
    runclone();
}