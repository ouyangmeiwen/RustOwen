mod db;

use db::Database;  // 只需要引入 Database
use std::io::{self, Write};

fn main() {
    // 连接数据库（如果数据库文件不存在会创建）
    let db = Database::new("users.db").expect("数据库连接失败");
    
    // 创建用户表（如果没有的话）
    db.create_table().expect("创建表格失败");

    loop {
        println!("\n选择操作:");
        println!("1. 添加用户");
        println!("2. 查看所有用户");
        println!("3. 更新用户");
        println!("4. 删除用户");
        println!("5. 退出");

        let mut choice = String::new();
        print!("请输入选择 (1-5): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_user(&db),
            "2" => view_users(&db),
            "3" => update_user(&db),
            "4" => delete_user(&db),
            "5" => {
                println!("退出程序...");
                break;
            }
            _ => println!("无效选择，请重新输入。"),
        }
    }
}

// 添加用户
fn add_user(db: &Database) {
    let mut name = String::new();
    let mut email = String::new();

    print!("请输入用户姓名: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("请输入用户邮箱: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    let name = name.trim();
    let email = email.trim();

    match db.insert_user(name, email) {
        Ok(id) => println!("用户添加成功，ID: {}", id),
        Err(e) => eprintln!("添加用户失败: {}", e),
    }
}

// 查看所有用户
fn view_users(db: &Database) {
    match db.get_all_users() {
        Ok(users) => {
            if users.is_empty() {
                println!("没有用户。");
            } else {
                for user in users {
                    println!("{:?}", user);
                }
            }
        }
        Err(e) => eprintln!("查询用户失败: {}", e),
    }
}

// 更新用户
fn update_user(db: &Database) {
    let mut id = String::new();
    let mut name = String::new();
    let mut email = String::new();

    print!("请输入要更新的用户ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();

    print!("请输入新的用户姓名: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("请输入新的用户邮箱: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    let id: i32 = id.trim().parse().unwrap_or(0);
    let name = name.trim();
    let email = email.trim();

    match db.update_user(id, name, email) {
        Ok(rows_updated) => {
            if rows_updated > 0 {
                println!("用户信息更新成功！");
            } else {
                println!("用户ID未找到！");
            }
        }
        Err(e) => eprintln!("更新失败: {}", e),
    }
}

// 删除用户
fn delete_user(db: &Database) {
    let mut id = String::new();

    print!("请输入要删除的用户ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();

    let id: i32 = id.trim().parse().unwrap_or(0);

    match db.delete_user(id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                println!("用户删除成功！");
            } else {
                println!("用户ID未找到！");
            }
        }
        Err(e) => eprintln!("删除失败: {}", e),
    }
}
