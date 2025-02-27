# 使用官方的 Rust 镜像作为基础镜像
# FROM rust:1.70 as builder
FROM swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/rust:1.82.0 as builder

# 创建并设置工作目录
WORKDIR /app

# 将当前目录内容复制到容器的工作目录中
COPY . .

# 构建 Rust 项目
RUN cargo build --release

# 使用官方的 slim 镜像作为运行时镜像
# FROM debian:bullseye-slim
FROM swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/ubuntu:22.04

# 安装运行时所需的依赖
RUN apt-get update && apt-get install -y \
    libssl3  \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# 从构建阶段复制已构建的二进制文件
COPY --from=builder /app/target/release/rust-mysql-sqx-api /usr/local/bin/rust-mysql-sqx-api

# 复制 .env 文件到容器中 但是目前发现无法读取env下次再找原因
COPY --from=builder /app/.env /usr/local/bin/.env

COPY --from=builder /app/config.json /usr/local/bin/config.json
COPY --from=builder /app/config.yaml /usr/local/bin/config.yaml

# 设置容器的启动命令
CMD ["/usr/local/bin/rust-mysql-sqx-api"]
