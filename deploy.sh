set -e
set -x

cargo build
espflash save-image --chip esp32 target/xtensa-esp32-espidf/debug/rust-esp32-hello-ota target/to_deploy.bin
curl --data-binary @target/to_deploy.bin "http://$1/flash" -v
