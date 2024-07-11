## Auto Change Gogs Password

This tool automatically changes the password for a Gogs user. If executed a second time, it will revert the password back to its original state.

### Development Environment

- rustc: rustc 1.78.0-nightly
- system: windows,linux,mac
- runtime: windows,linux,mac

### Compilation

#### Install Docker

Refer to the [Docker documentation](https://docs.docker.com/get-docker/) for installation instructions.

#### Install Cross

Refer to the [Cross documentation](https://github.com/cross-rs/cross) for more details.

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

#### Compile to Target Platform

```bash
cross build --target x86_64-unknown-linux-musl --release
```

## Usage

```
gpasswd --host <HOST> -u <USERNAME> -p <PASSWORD> -t <TEMP_PASSWORD>
```

- `host`: Format like http://ip:port
- `u`: Username
- `p`: Password
- `t`: Temporary password