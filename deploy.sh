#!/usr/bin/zsh

cargo build --target=armv7-unknown-linux-gnueabihf
scp target/armv7-unknown-linux-gnueabihf/debug/parallel pi@raspberrypi:~/rust
scp config.json pi@raspberrypi:~/rust