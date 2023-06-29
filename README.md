# Requirements
- build-essential
- rust (nightly)
- GRUB
- xorriso
- QEMU
## Example install commands in ubuntu
In this directory,
```bash
# build-essential
sudo apt install build-essential
# rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# rust nightly
rustup toolchain install nightly
rustup override set nightly
# library for rust
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
# GRUB
sudo apt install grub-common
# xorriso
sudo apt install xorriso
# QEMU
sudo apt install qemu-system-i386
```

# Build and Run
```
make run
```
## clean
```
make clean
```
## Run with GDB
```
make run-gdb
```
In another terminal,
```
gdb -x gdb_init
```


# Reference
- https://itto-ki.hatenablog.com/entry/2018/08/06/020220
- https://github.com/itto-ki/SawayakanaOS
- https://pdos.csail.mit.edu/6.828/2018/schedule.html
- https://github.com/utam0k/nyarnos