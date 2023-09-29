# rust-esp32-hello-ota
A hello world for the ESP32 with OTA (HTTP) firmware updates.

ESP32 Dev Env Setup
-----
Notes on approx. how I got it working under Ubuntu 23.04.

```
sudo apt install -y git curl gcc clang ninja-build cmake libudev-dev \
                    unzip xz-utils python3 python3-pip python3-venv \
                    libusb-1.0-0 libssl-dev pkg-config
sudo snap install rustup --classic
rustup install stable
cargo install espup
espup install
rustup toolchain link esp ~/.rustup/toolchains/esp
. $HOME/export-esp.sh
```

Use
---
Set `SSID` and `PASS` in `main.rs`.

Build and deploy over serial once:
```
cargo espflash flash -p /dev/ttyUSB0
espflash monitor
```
Note the IP in the serial logs.

For future builds, run: `deploy.sh <ip>` *Deplay will not exit, but it will be obvious when done.  Just ctrl-c.*

To revert to the last build, run: `curl -X POST http://10.0.0.49/revert`
