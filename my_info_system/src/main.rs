mod db; // 导入 db 模块

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read,Write};
use std::env;
use db::Database; // 假设 Database 在 db 模块中定义
fn main() {
    // Set the locale for the application
    env::set_var("LANG", "en_US.UTF-8");

    // Connect to the database (it will create the file if not exists)
    let db = Database::new("users.db").expect("Failed to connect to the database");

    // Create user table if not exists
    db.create_table().expect("Failed to create the table");

    // Start the TCP server in a new thread
    thread::spawn(move || {
        start_socket_server();
    });

    // Main menu loop for interacting with the database
    loop {
        println!("\nChoose an operation:");
        println!("1. Add user");
        println!("2. View all users");
        println!("3. Update user");
        println!("4. Delete user");
        println!("5. Exit");

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
                println!("Exiting the program...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

// Add user
fn add_user(db: &Database) {
    let mut name = String::new();
    let mut email = String::new();

    print!("Enter user name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Enter user email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    let name = name.trim();
    let email = email.trim();

    match db.insert_user(name, email) {
        Ok(id) => println!("User added successfully, ID: {}", id),
        Err(e) => eprintln!("Failed to add user: {}", e),
    }
}

// View all users
fn view_users(db: &Database) {
    match db.get_all_users() {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found.");
            } else {
                for user in users {
                    println!("{:?}", user);
                }
            }
        }
        Err(e) => eprintln!("Failed to query users: {}", e),
    }
}

// Update user
fn update_user(db: &Database) {
    let mut id = String::new();
    let mut name = String::new();
    let mut email = String::new();

    print!("Enter user ID to update: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();

    print!("Enter new user name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Enter new user email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    let id: i32 = id.trim().parse().unwrap_or(0);
    let name = name.trim();
    let email = email.trim();

    match db.update_user(id, name, email) {
        Ok(rows_updated) => {
            if rows_updated > 0 {
                println!("User information updated successfully!");
            } else {
                println!("User ID not found!");
            }
        }
        Err(e) => eprintln!("Update failed: {}", e),
    }
}

// Delete user
fn delete_user(db: &Database) {
    let mut id = String::new();

    print!("Enter user ID to delete: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();

    let id: i32 = id.trim().parse().unwrap_or(0);

    match db.delete_user(id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                println!("User deleted successfully!");
            } else {
                println!("User ID not found!");
            }
        }
        Err(e) => eprintln!("Delete failed: {}", e),
    }
}

// Start the socket server
fn start_socket_server() {
    let listener = TcpListener::bind("127.0.0.1:7777").expect("Failed to bind socket");
    println!("Socket server started on port 7777...");

    // Loop to accept incoming TCP connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
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
