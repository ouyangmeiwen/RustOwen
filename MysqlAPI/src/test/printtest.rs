use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person: {} (age {})", self.name, self.age)
    }
}

pub fn print_all_formats() {
    let integer = 42;
    let float: f64 = 3.14159;
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 基本格式化
    println!("Basic format: {}", integer);

    // 格式化浮点数
    println!("Floating point (2 decimal places): {:.2}", float); // 输出: 3.14
    println!("Floating point (scientific): {:e}", float); // 输出: 3.141590e0
                                                          //println!("Floating point (fixed-point): {:f}", float); // 输出: 3.141590

    // 宽度控制
    println!("Width control (right): {:5}", integer);
    println!("Width control (left): {:<5}", integer);
    println!("Width control (center): {:^5}", integer);

    // 填充字符
    println!("Filled (0 padding): {:0>5}", integer);
    println!("Filled (* padding): {:*<5}", integer);

    // 二进制、八进制、十六进制
    println!("Binary: {:b}", integer);
    println!("Octal: {:o}", integer);
    println!("Hexadecimal (lower): {:x}", integer);
    println!("Hexadecimal (upper): {:X}", integer);

    // 使用位置参数
    println!("Positioned (first and second): {0} {1}", integer, float);

    // 使用命名参数
    println!(
        "Named parameters: {number} {word}",
        number = integer,
        word = "hello"
    );

    // 调试输出
    println!("Debug format: {:?}", person);
    println!("Pretty Debug format: {:#?}", person);

    // 自定义 Display 格式
    println!("Custom Display format: {}", person);
}
