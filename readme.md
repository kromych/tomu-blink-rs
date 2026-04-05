# Example applications for the Tomu board

## The board

This is a miniature board that fits into the USB port. Here are the details:
https://tomu.im/tomu.html.

## Demo's

TBD

## Prepare

One-time tool installation:

```sh
cargo install flip-link
cargo install probe-rs --features="cli"
cargo install cargo-binutils
cargo install coreutils
cargo install cargo-dfu
```

## Flashing and running

These steps are handled by `cargo run` and a custom runner, e.g.:

```sh
cargo run --release --bin blink
```

will run the `blink` demo.

Note that by default, the bootloader **will always run** when the board
is powered on. This makes easier to develop software, because all you need
to do to load new firmware is to unplug Tomu and plug it back in.

To automatically run your program at power on, use `tomu::toboot_config!`
and enable auto-run. That'll make development a bit less convenient as you'll
need to touch the capacitive buttons before applying power to get into
the bootloader when need to re-flash. The **nuclear option** is to lock
yourself (and everyone else) out of the bootloader so Tomu will always
run only what's in flash. See [`toboot.rs`](./support/tomu/src/toboot.rs)
for the details of that.
