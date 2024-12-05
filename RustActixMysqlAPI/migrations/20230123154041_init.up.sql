-- Add migration script here

CREATE TABLE IF NOT EXISTS notes (
    id CHAR(36) PRIMARY KEY NOT NULL DEFAULT (UUID()),  -- 使用 UUID() 生成 UUID 字符串
    title VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    category VARCHAR(100),
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,   -- 使用 CURRENT_TIMESTAMP 作为默认值
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP  -- 自动更新为当前时间戳
);
