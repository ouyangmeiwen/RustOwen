mod db; // 导入 db 模块
mod db_operations; // 导入 db_operations 模块

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read, Write};
use std::env;
use db::Database; // 假设 Database 在 db 模块中定义
use db_operations::{add_user, view_users, update_user, delete_user}; // 导入 db_operations 中的函数

fn main() {
    // Set the locale for the application
    env::set_var("LANG", "en_US.UTF-8");

    // Connect to the database (it will create the file if not exists)
    let db = Database::new("users.db").expect("Failed to connect to the database");

    // Create user table if not exists
    db.create_table().expect("Failed to create the table");
     {
        println!("\nChoose an operation:");
        println!("1. Database operations");
        println!("2. Start socket server");
        println!("5. Exit");

        let mut choice = String::new();
        print!("Enter your choice (1 for database, 2 for socket server, 5 to exit): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                // Enter the database operations menu
                database_operations(&db);
            },
            "2" => {
                // Start the TCP server when user chooses option 2
                start_socket_server();
            },
            "5" => {
                println!("Exiting the program...");
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

// Database operations menu
fn database_operations(db: &Database) {
    loop {
        println!("\nChoose a database operation:");
        println!("1. Add user");
        println!("2. View all users");
        println!("3. Update user");
        println!("4. Delete user");
        println!("5. exit");

        let mut choice = String::new();
        print!("Enter your choice (1-5): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_user(&db),
            "2" => view_users(&db),
            "3" => update_user(&db),
            "4" => delete_user(&db),
            "5" => {
                println!("Returning to the main menu...");
                break; // Exit the database menu
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

// Start the socket server
fn start_socket_server() {
    println!("Starting the socket server...");
    let listener = TcpListener::bind("127.0.0.1:7777").expect("Failed to bind socket");

    // Loop to accept incoming TCP connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each client connection in a new thread
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

// Handle each client connection
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        // Read the message from the client
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed by client
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received message: {}", message);

                // Send the same message back to the client
                if let Err(e) = stream.write_all(message.as_bytes()) {
                    eprintln!("Failed to send message: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}
