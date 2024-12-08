use std::vec;

struct Body {
    title: Option<String>,
}

fn runtest1() {
    let body: Body = Body {
        title: Some("Hello1".to_string()),
    };
    // 通过 unwrap 获取所有权
    let title: String = body.title.unwrap();
    println!("{}", title); // 输出: Hello
                           //println!("{}", body.title.unwrap());  // 错误: `body.Title` 的所有权已被移动
}
fn runtest2() {
    let body1: Body = Body {
        title: Some("Hello2".to_string()),
    };
    let some_body = body1.title.as_ref().unwrap();
    println!("{}", some_body); // 输出: Hello
}
fn runtest3() {
    let mut body2: Body = Body {
        title: Some("Hello3".to_string()),
    };
    // 使用可变引用来修改 Title 字段的值
    if let Some(title2) = body2.title.as_mut() {
        *title2 = "Hello31".to_string(); // 修改值
        println!("{}", title2); // 输出: Hello
    }
    println!("{}", body2.title.as_ref().unwrap()); // 输出: Hello
}

fn runtest4() {
    let hello_string = "hello4".to_string(); // 创建一个长生命周期的 String
    let ts = Some(&hello_string); // 使用 hello_string 的引用
    println!("{}", ts.unwrap()); // 现在引用没有问题
}

fn runtest5() {
    let hello_string = "hello5".to_string(); // 创建一个长生命周期的 String
    let ts = Some(hello_string); // 使用 hello_string 的引用
    println!("{}", ts.unwrap()); // 现在引用没有问题
                                 //println!("{}", hello_string); // 现在引用没有问题 报错
                                 //println!("{}", ts.unwrap()); // 现在引用没有问题  报错
}
fn runtest6() {
    let hello_string = "hello6".to_string(); // 创建一个长生命周期的 String
    let ts = Some(hello_string); // 使用 hello_string 的引用
    println!("{}", ts.as_ref().unwrap()); // 现在引用没有问题
                                          //println!("{}", hello_string); // 现在引用没有问题  报错
    println!("{}", ts.as_ref().unwrap()); // 现在引用没有问题
}

fn runmove() {
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

fn runcopy() {
    let x = 5; // x 是 i32 类型，类型 i32 实现了 Copy
    let y = x; // 这时发生的是值的复制，而不是所有权转移
    println!("{}", x); // x 仍然有效，可以继续使用
    println!("{}", y); // y 也有效，拥有值 5
}

fn runborrow() {
    let a = String::from("hello"); // a 拥有 String 的所有权
    let b = &a; // b 是对 a 的不可变引用，所有权没有转移
    println!("{}", a); // a 仍然有效
    println!("{}", b); // b 也可以访问 a 的内容
}
fn runclone() {
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
fn find_item(id: u32) -> Option<String> {
    if id == 1 {
        Some("Item found".to_string())
    } else {
        None
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string()) // 错误情况
    } else {
        Ok(a / b) // 成功情况
    }
}

fn id_in_options(items: &Vec<i32>, id: i32) -> Option<i32> {
    if items.contains(&id) {
        Some(id)
    } else {
        None
    }
}

fn id_in_results(items: &Vec<i32>, id: i32) -> Result<i32, String> {
    if items.contains(&id) {
        Ok(id)
    } else {
        Err((&"NoteFoud").to_string())
    }
}

pub fn runtest() {
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

    match find_item(1) {
        Some(item) => println!("Found: {}", item),
        None => println!("Item not found"),
    }
    if let Some(item) = find_item(1) {
        println!("Found: {}", item);
    } else {
        println!("Item not found");
    }

    match divide(10, 2) {
        Ok(value) => println!("Result is: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let result = divide(10, 0);
    if let Ok(value) = result {
        println!("Result is: {}", value);
    } else if let Err(e) = result {
        println!("Error: {}", e);
    }

    let result = divide(10, 2);
    // 对 Ok 值进行操作
    let doubled = result.map(|x| x * 2);
    println!("Doubled result: {:?}", doubled);
    // 对 Err 值进行操作
    let result_with_error = divide(10, 0);
    let mapped_error = result_with_error.map_err(|e| format!("Custom error: {}", e));
    println!("Mapped error: {:?}", mapped_error);

    let result = divide(10, 0).unwrap_or_else(|e| {
        println!("An error occurred: {}", e);
        -1 // 返回一个默认值
    });
    println!("Result is: {}", result);

    let items = vec![1, 2, 3, 4, 5]; // 使用 vec! 宏创建并初始化 Vec
    let id = 10;

    match id_in_options(&items, id) {
        Some(found_id) => println!("Found: {}", found_id),
        None => println!("Not found"),
    }
    match id_in_results(&items, id) {
        Ok(id) => {
            println!("found {}", id);
        }
        Err(err) => {
            println!("err:{}", err);
        }
    }
    println!("{}", id_in_options(&items, id).unwrap_or(-1));
    println!("{}", id_in_options(&items, id).as_ref().unwrap_or(&-1));

    println!("{}", id_in_results(&items, id).unwrap_or(-1));
    println!("{}", id_in_results(&items, id).as_ref().unwrap_or(&-1));

    println!(
        "{}",
        id_in_options(&items, id).unwrap_or_else(|| {
            println!("Error occurred"); // 打印错误信息
            -1 // 返回默认值
        })
    );
    println!(
        "{}",
        id_in_options(&items, id).as_ref().unwrap_or_else(|| {
            println!("Error occurred"); // 打印错误信息
            &-1 // 返回默认值
        })
    );
    println!(
        "{}",
        id_in_results(&items, id).unwrap_or_else(|err| {
            println!("Error occurred: {}", err); // 打印错误信息
            -1 // 返回默认值
        })
    );
    println!(
        "{}",
        id_in_results(&items, id).as_ref().unwrap_or_else(|err| {
            println!("Error occurred: {}", err); // 打印错误信息
            &-1 // 返回默认值
        })
    );
}
