# ctp-rs
A simple demo for use bindgen with ctp sdk, use at your own risk. Config `CROSS_CTP_SDK_PATH` to the sdk folder in `.cargo/config.toml` first.

```bash
# build for host
cargo build

# build for x86_64-unknown-linux-gnu
cross build --target=x86_64-unknown-linux-gnu --target-dir ./target/x86_64-unknown-linux-gnu -vvv

# build for x86_64-pc-windows-gnu
cross build --target=x86_64-pc-windows-gnu --target-dir ./target/build-win-x86_64 -vvv

# build for x86_64-pc-windows-msvc
# NOTE: need build docker image by yourself
cross build --target=x86_64-pc-windows-msvc --target-dir ./target/x86_64-pc-windows-msvc -vvv

# build for aarch64-apple-darwin
# NOTE: need build docker image by yourself
cross build --target=aarch64-apple-darwin --target-dir ./target/build-apple-aarch64 -vvv
```
