# Procedure

## Install Dependencies

```sh
yay -S avr-gcc avr-libc avrdude
cargo install cargo-generate
cargo install ravedude
```

## Take Arduino ownership

```sh
sudo chmod a+rw /dev/ttyACM0
```

## Run

```sh
cargo run
```
