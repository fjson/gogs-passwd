### 开发环境

- rustc: rustc 1.78.0-nightly
- system: windows,linux,mac
- runtime: windows,linux,mac

### 编译

#### 安装docker

[docker文档](https://docs.docker.com/get-docker/)

#### 安装cross

[cross 文档](https://github.com/cross-rs/cross)

```bash
cargo install cross --git https://github.com/cross-rs/cross
```
#### 编译至目标平台

```bash
cross build --target x86_64-unknown-linux-musl --release
```

## 使用方式

```
cargo run -- -u <用户名> -p <密码> -t <临时密码>
```