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
cargo install cargo-espflash
cargo install espup
echo 'export PATH="$PATH:~/.cargo/bin"' >> ~/.bashrc
source ~/.bashrc
espup install
. $HOME/export-esp.sh
```

Project Init
------------
```
cargo new <project-name>
```

Create `Cargo.toml`:
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Use
---
Set `SSID` and `PASS` in `main.rs`.

Build and deploy over serial once:
```
cargo espflash flash -p /dev/ttyUSB0
espflash monitor
```
Note the IP in the serial logs.  Ex:
```
IP info: IpInfo { ip: 10.0.0.49, subnet: Subnet { gateway: 10.0.0.1, mask: Mask(24) }, dns: Some(75.75.75.75), secondary_dns: Some(75.75.76.76) }
```

For future builds, run: `deploy.sh <ip>`

*Deploy will not exit, but it will be obvious when done.  Just ctrl-c. Ex:*
```
* We are completely uploaded and fine
< HTTP/1.1 200 OK
```

To revert to the last build, run: `curl -X POST http://10.0.0.49/revert`
