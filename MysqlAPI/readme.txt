调试F5

cargo build


发布:
cargo build --release



开启rabbitmq 管理页
rabbitmq-plugins enable rabbitmq_management
rabbitmqctl start





















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