cargo build --release

##### Directly Erase & Flash the firmware to board
probe-rs download --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/ZM9K-BLE


- [x] Works with custom nice!nano derived bootloader with s140
- [ ] bbb

# Download Nordic's MBR
curl -L https://github.com/alexmoon/nrf-sdc/raw/main/firmware/mbr_nrf52_2.4.1_mbr.hex -o mbr.hex

# Flash MBR (probe-rs auto-detects .hex format)
probe-rs download --chip nRF52840_xxAA mbr.hex



# Clean Builds Artifacts
```
cargo clean
```


# Erase, Build, Flash and Attach to RTT logs

## completely erase the flash
probe-rs erase --chip nRF52840_xxAA --allow-erase-all

## Then flash your application
cargo build --release
probe-rs download --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/ZM9K-BLE


## Attach to RTT logs withouth reflashing
probe-rs attach --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/ZM9K-BLE


# Build and Generate .UF2

```
# Build
cargo build --release
# Generate .UF2
cargo make uf2 --release
```


# Findings

## Flash nRF MBR when no bootloader is present

Download first:

Flash:
```
probe-rs download --chip nRF52840_xxAA --binary-format bin --base-address 0x0 mbr_nrf52_2.4.1_mbr.bin
```

## When your complelty locked out of the nRF52840

```
nrfjprog --recover
```

## Loads into RAM not Flash
Application no longer works after power cycle
```
probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/ZM9K-BLE
```

## Loads into Flash


## Error 7 Nov 2025

Running Command:
```
probe-rs attach --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/ZM9K-BLE
```

Error:
```
[WARN ] Memory buffer too big. 2816 bytes needed (nrf_sdc nrf-sdc/src/fmt.rs:151)
[INFO ] Flash capacity 1024 KB, RMK use 128 KB(32 sectors) starting from 0xA0000 as storage (rmk src/storage/mod.rs:793)
[ERROR] panicked at 'MultiprotocolServiceLayer: 106:519' (nrf_mpsl nrf-mpsl/src/fmt.rs:101)
[ERROR] panicked at /Users/alexzidros/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/defmt-1.0.1/src/lib.rs:385:5:
explicit panic (panic_probe panic-probe-1.0.0/src/lib.rs:104)
```

## NRF-SDC