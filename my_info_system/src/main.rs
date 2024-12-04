mod db;

use db::Database;  // Only need to import Database
use std::io::{self, Write};
use std::env;

fn main() {
    env::set_var("LANG", "en_US.UTF-8");
    // Connect to the database (it will create the file if not exists)
    let db = Database::new("users.db").expect("Failed to connect to the database");

    // Create user table if not exists
    db.create_table().expect("Failed to create the table");

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
