use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    // 连接到数据库（创建数据库文件）
    pub fn new(db_file: &str) -> Result<Self> {
        let conn = Connection::open(db_file)?;
        Ok(Database { conn })
    }

    // 创建用户表
    pub fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    // 插入新用户
    pub fn insert_user(&self, name: &str, email: &str) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO users (name, email) VALUES (?1, ?2)",
            params![name, email],
        )?;
        Ok(self.conn.last_insert_rowid() as i32)
    }

    // 获取所有用户
    pub fn get_all_users(&self) -> Result<Vec<User>> {
        let mut stmt = self.conn.prepare("SELECT id, name, email FROM users")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
            })
        })?;

        let mut users = Vec::new();
        for user in user_iter {
            users.push(user?);
        }
        Ok(users)
    }

    // 更新用户信息
    pub fn update_user(&self, id: i32, name: &str, email: &str) -> Result<usize> {
        self.conn.execute(
            "UPDATE users SET name = ?1, email = ?2 WHERE id = ?3",
            params![name, email, id],
        )
    }

    // 删除用户
    pub fn delete_user(&self, id: i32) -> Result<usize> {
        self.conn.execute("DELETE FROM users WHERE id = ?1", params![id])
    }
}
