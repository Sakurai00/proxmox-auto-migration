# Auto migration

[![CI](https://github.com/Sakurai00/Auto-migration/actions/workflows/CI.yml/badge.svg)](https://github.com/Sakurai00/Auto-migration/actions/workflows/CI.yml)

## 使い方

```sh
cross build --target=aarch64-unknown-linux-gnu --release --target-dir ./target
scp ./target/aarch64-unknown-linux-gnu/release/auto_migration root@RPI01-pve:~/
```
