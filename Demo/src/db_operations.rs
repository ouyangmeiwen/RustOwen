// db_operations.rs

use crate::db::Database;
use std::io::{self, Write};
// Add user
pub fn add_user(db: &Database) {
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
pub fn view_users(db: &Database) {
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
pub fn update_user(db: &Database) {
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
pub fn delete_user(db: &Database) {
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
