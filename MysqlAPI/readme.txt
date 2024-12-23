调试F5

cargo build


发布:
cargo build --release



开启rabbitmq 管理页
rabbitmq-plugins enable rabbitmq_management
rabbitmqctl start



借用权的操作
借用权允许你在不获取所有权的情况下访问数据，可以是不可变借用或可变借用。
不可变借用 (&)
    当你通过不可变引用借用数据时，你无法修改数据，但可以读取它。多个不可变借用可以同时存在：
可变借用 (&mut)
    当你通过可变引用借用数据时，你可以修改数据，但在某一时间内只能有一个可变引用。你不能在同一作用域内同时有不可变和可变借用：
其他注意事项
    切片：切片也是一种借用的方式，尤其在操作集合（如字符串、数组、切片）时。
    函数参数：当你把一个值传递给函数时，如果传递的是引用（而非直接传值），那么就是借用。
获得所有权：通过赋值、函数返回、集合元素移动等操作。
借用权：通过不可变引用（&）和可变引用（&mut）获得。






获取所有权的操作
    通过变量赋值（转移所有权）
    当一个值赋给另一个变量时，所有权会从源变量转移到目标变量，源变量不再拥有该值：

    通过函数返回值（转移所有权）
    当一个值被从函数返回时，所有权会转移到函数的调用者：

    通过Box<T>进行所有权转移 不可做共享
    Box<T>是一个堆分配的智能指针，当你将一个值放入 Box 中时，所有权转移到 Box：

    Arc<T> 获取所有权的操作
    与 Box<T> 类似，当你将一个值放入 Arc<T> 中时，所有权会从原始变量转移到 Arc<T>：
    克隆 Arc<T>（增加引用计数）
    由于 Arc<T> 使用引用计数，所有者可以在不同线程中共享所有权，适合多线程环境。每次克隆 Arc<T>，都会共享堆上的数据，并增加引用计数。

    通过Vec<T>的元素移动
    如果你从一个 Vec 中移除元素，元素的所有权将转移到新的变量：

    通过String类型的操作
    对于 String 类型的值，如果它被赋给另一个 String 类型变量，所有权会转移：

    通过std::mem::replace替换值
    使用 std::mem::replace 也会将值的所有权转移：

    通过Option<T>或Result<T, E>等枚举的值转移
    枚举类型（如 Option<T> 和 Result<T, E>）可以存储一个值，当从中解构出值时，所有权会转移：

    通过移动到线程中的数据
    当你将数据传递到一个新线程时，数据的所有权会转移到该线程：
    
    通过std::fs::File等类型
    类似于文件、网络连接等资源，传递给这些类型时，资源的所有权会转移：



--gdb


{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug executable 'rust-mysql-sqx-api'",
            "type": "cppdbg",  // Change type to cppdbg
            "request": "launch",
            "program": "${workspaceFolder}/target/debug/rust-mysql-sqx-api",  // Path to your Rust executable
            "args": [],
            "stopAtEntry": true,
            "cwd": "${workspaceFolder}",
            "environment": [],
            "externalConsole": false,
            "MIMode": "gdb",  // Use gdb for debugging
            "miDebuggerPath": "gdb",  // Path to GDB executable
            "setupCommands": [
                {
                    "description": "Enable pretty-printing for gdb",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                }
            ],
            "logging": { "engineLogging": true }
        },
        {
            "name": "Debug unit tests in executable 'rust-mysql-sqx-api'",
            "type": "cppdbg",  // Change type to cppdbg
            "request": "launch",
            "program": "${workspaceFolder}/target/debug/rust-mysql-sqx-api",  // Path to your Rust executable
            "args": ["--test"],  // Optionally, specify test arguments if needed
            "stopAtEntry": true,
            "cwd": "${workspaceFolder}",
            "environment": [],
            "externalConsole": false,
            "MIMode": "gdb",  // Use gdb for debugging
            "miDebuggerPath": "gdb",  // Path to GDB executable
            "setupCommands": [
                {
                    "description": "Enable pretty-printing for gdb",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                }
            ],
            "logging": { "engineLogging": true }
        }
    ]
}







 
--lldb 官方推荐


{
    // 使用 IntelliSense 了解相关属性。 
    // 悬停以查看现有属性的描述。
    // 欲了解更多信息，请访问: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'rust-mysql-sqx-api'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=rust-mysql-sqx-api",
                    "--package=rust-mysql-sqx-api"
                ],
                "filter": {
                    "name": "rust-mysql-sqx-api",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'rust-mysql-sqx-api'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=rust-mysql-sqx-api",
                    "--package=rust-mysql-sqx-api"
                ],
                "filter": {
                    "name": "rust-mysql-sqx-api",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}