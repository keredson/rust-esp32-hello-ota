set -e
set -x

cargo build
espflash save-image --chip esp32 target/xtensa-esp32-espidf/debug/rust-blink-182 target/to_deploy.bin
curl --data-binary @target/to_deploy.bin http://10.0.0.49/flash -v
