# 使用官方 Rust 镜像
FROM rust:latest as builder

# 创建并进入工作目录
WORKDIR /usr/src/app

# 将代码拷贝到容器中
COPY . .

# 编译项目
RUN cargo build --release

# 生成运行时镜像
FROM debian:buster-slim

# 安装依赖库（例如 libc）
RUN apt-get update && apt-get install -y libssl-dev

# 拷贝编译好的二进制文件
COPY --from=builder /usr/src/app/target/release/your_project_name /usr/local/bin/

# 运行程序
CMD ["actix_web_api"]
