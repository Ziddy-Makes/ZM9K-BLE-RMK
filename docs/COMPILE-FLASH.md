# Compile and Flash Guide for ZM9K-BLE (nRF52840)

This guide covers how to compile and flash the ZM9K-BLE firmware to an nRF52840 microcontroller using two methods:
1. **SWD with Programmer** - Using a debugging probe (recommended for development)
2. **UF2 with Bootloader** - Using the Adafruit nRF52 Bootloader (easiest method)

## Prerequisites

### Required Tools

1. **Rust Toolchain** - Install Rust and the required target:
   ```bash
   rustup target add thumbv7em-none-eabihf
   ```

2. **cargo-make** - For building and generating firmware:
   ```bash
   cargo install --force cargo-make
   ```

### For SWD Flashing

3. **probe-rs** - For flashing via SWD:
   ```bash
   cargo install probe-rs --locked --features cli
   ```

4. **Hardware Requirements:**
   - SWD debugging probe (e.g., J-Link, Black Magic Probe, or any probe-rs compatible probe)
   - Connect SWDIO (SWD Data), SWCLK (SWD Clock), GND, and optionally VCC to your nRF52840 board

### For UF2 Flashing

3. **Bootloader** - Your board must have the Adafruit nRF52 Bootloader installed (pre-installed on nice!nano and compatible boards)

---

## Compilation

### Build the Firmware

Compile the firmware in release mode:

```bash
cargo make build --release
```

Or using cargo directly:

```bash
cargo build --release
```

This will create the firmware binary at:
```
target/thumbv7em-none-eabihf/release/ZM9K-BLE
```

### Generate HEX File

To generate a `.hex` file (needed for both flashing methods):

```bash
cargo make objcopy --release
```

This creates `ZM9K-BLE.hex` in the project root.

---

## Flashing Methods

### Method 1: SWD with Programmer (probe-rs)

This method uses a debugging probe connected via SWD (Serial Wire Debug) interface. It's recommended for development and when you need to flash firmware without a bootloader.

#### Step 1: Connect Your Probe

Connect your SWD debugging probe to the nRF52840:
- **SWDIO** → SWD Data pin
- **SWCLK** → SWD Clock pin  
- **GND** → Ground
- **VCC** → Power (if your probe supplies power, otherwise connect to your board's power supply)

#### Step 2: List Available Probes

Check if probe-rs can detect your probe:

```bash
probe-rs list
```

You should see your probe listed. If you have multiple probes, note the probe identifier.

#### Step 3: Flash the Firmware

**Option A: Flash from ELF file (Recommended)**
```bash
probe-rs download --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/ZM9K-BLE
```

This is the preferred method as probe-rs automatically detects the ELF format and handles all sections correctly.

**Option B: Flash from HEX file**
```bash
probe-rs download --chip nRF52840_xxAA ZM9K-BLE.hex
```

probe-rs will automatically detect the HEX format from the file extension.

**Option C: Flash from binary file**
If you need to flash a raw binary file, specify the format and base address:
```bash
probe-rs download --chip nRF52840_xxAA --binary-format bin --base-address 0x0 target/thumbv7em-none-eabihf/release/ZM9K-BLE.bin
```

**Option D: Flash and run (for development)**
```bash
cargo run --release
```

This uses the runner configured in `.cargo/config.toml` which will automatically flash and run the firmware.

#### Step 4: Verify Flash

To verify the flash was successful, you can read back the firmware:

```bash
probe-rs read --chip nRF52840_xxAA --address 0x0 --length 0x1000
```

#### Troubleshooting SWD Flashing

- **Probe not detected**: Ensure your probe drivers are installed and the probe is connected via USB
- **Connection timeout**: Check your SWD connections (SWDIO, SWCLK, GND)
- **Chip not detected**: Verify the chip identifier matches `nRF52840_xxAA`. Some variants may use different identifiers
- **Permission errors** (Linux): Add your user to the `plugdev` group:
  ```bash
  sudo usermod -a -G plugdev $USER
  # Log out and back in for changes to take effect
  ```

---

### Method 2: UF2 with Bootloader

This method uses the Adafruit nRF52 Bootloader and is the easiest way to flash firmware without a debugging probe. It's commonly used on boards like the nice!nano.

#### Step 1: Generate UF2 File

Generate the UF2 firmware file:

```bash
cargo make uf2 --release
```

This command will:
1. Build the firmware (`cargo make build --release`)
2. Convert to HEX format (`cargo make objcopy`)
3. Convert HEX to UF2 format (`cargo make uf2`)

The output file `ZM9K-BLE.uf2` will be created in the project root.

#### Step 2: Enter Bootloader Mode

Put your board into bootloader mode. The method varies by board:

- **nice!nano**: Press the reset button twice quickly, or short the GND and RST pins twice
- **Other boards**: Consult your board's documentation for bootloader entry method

When in bootloader mode, a USB drive named something like `BOOT` or `NRF52BOOT` should appear on your computer.

#### Step 3: Flash the Firmware

Simply drag and drop the `ZM9K-BLE.uf2` file onto the bootloader USB drive. The firmware will automatically flash and the board will reset.

#### Step 4: Verify

After flashing, the board should automatically reset and run the new firmware. The bootloader USB drive will disappear, and your keyboard should be functional.

#### Troubleshooting UF2 Flashing

- **Bootloader USB drive doesn't appear**: 
  - Ensure you're in bootloader mode (try entering bootloader mode again)
  - Check USB cable connection
  - Try a different USB port/cable
  
- **UF2 file doesn't flash**:
  - Verify the file is `ZM9K-BLE.uf2` and not corrupted
  - Ensure the bootloader is compatible (Adafruit nRF52 Bootloader)
  - Try regenerating the UF2 file

- **Firmware doesn't work after flashing**:
  - Check that the firmware was compiled correctly
  - Verify your `keyboard.toml` configuration matches your hardware
  - Try re-flashing the firmware

---

## Additional Notes

### Development vs Release Builds

- **Debug build**: `cargo build` - Faster compilation, larger binary, includes debug symbols
- **Release build**: `cargo build --release` - Optimized for size and performance, recommended for production

### Multiple Probe Support

If you have multiple probes connected, specify which one to use:

```bash
probe-rs download --chip nRF52840_xxAA --probe <probe-id> ZM9K-BLE.hex
```

Or with ELF file:
```bash
probe-rs download --chip nRF52840_xxAA --probe <probe-id> target/thumbv7em-none-eabihf/release/ZM9K-BLE
```

### SoftDevice Compatibility

Since RMK v0.7.x, the firmware removes the old SoftDevice Bluetooth stack and uses its own implementation. If you need to rollback to v0.6.x or switch to firmware that uses SoftDevice (like ZMK), you may need to re-flash the bootloader.

For nice!nano users, see: [nice!nano Bootloader Flashing Guide](https://nicekeyboards.com/docs/nice-nano/troubleshooting#my-nicenano-seems-to-be-acting-up-and-i-want-to-re-flash-the-bootloader)

### USB vs BLE Mode

After flashing, RMK defaults to USB-priority mode if a USB cable is connected. To use BLE mode:
- Disconnect the USB cable, or
- Press the User11 (Switch Output) key to switch to BLE-priority mode

---

## Quick Reference

### SWD Flashing (Quick)
```bash
# Build and flash
cargo run --release
```

### UF2 Flashing (Quick)
```bash
# Generate UF2 and flash via bootloader
cargo make uf2 --release
# Then drag ZM9K-BLE.uf2 to bootloader USB drive
```

### Build Only
```bash
cargo make build --release
```

