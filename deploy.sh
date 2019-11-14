#!/usr/bin/zsh

cargo build --target=armv7-unknown-linux-gnueabihf
scp target/armv7-unknown-linux-gnueabihf/debug/parallel pi@10.0.170.224:~/rust
scp config.json pi@10.0.170.224:~/rust