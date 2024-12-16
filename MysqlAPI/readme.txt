调试F5

cargo build


发布:
cargo build --release



开启rabbitmq 管理页
rabbitmq-plugins enable rabbitmq_management
rabbitmqctl start
























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
