//! Toboot V2 bootloader support.
//!
//! # Overview
//!
//! Toboot is a DFU-based bootloader for the EFM32HG Tomu board. It occupies the
//! first 16 KiB of flash (pages 0-15, addresses 0x0000-0x3FFF). Applications
//! are placed starting at page 16 (0x4000).
//!
//! Source: <https://github.com/im-tomu/toboot>
//!
//! # Flash image layout
//!
//! The application binary has the following layout starting at 0x4000:
//!
//! ```text
//! Offset  Size  Content
//! ------  ----  -------
//! 0x000   0x04  Initial stack pointer (must point into RAM 0x2000_0000..0x2000_2000)
//! 0x004   0x04  Reset vector (must point into app flash 0x4000..0x10000)
//! 0x008   0x38  Exception vectors (14 entries)
//! 0x040   0x54  Interrupt vectors (21 entries on EFM32HG309)
//! 0x094   0x18  Toboot V2 configuration header (24 bytes)
//! 0x0AC   ...   .text section
//! ```
//!
//! The `.toboot` linker section (defined in `device.x` via `INSERT AFTER
//! .vector_table`) places the configuration header at offset 0x94. The
//! `memory.x` file sets `_stext = ORIGIN(FLASH) + 0xAC` so that `.text`
//! starts immediately after.
//!
//! The `EXTERN(__TOBOOT_CONFIG)` directive in `memory.x` forces the linker
//! to retain the symbol even though nothing references it from code.
//!
//! # Configuration header
//!
//! ```text
//! Offset  Size  Field           Description
//! ------  ----  -----           -----------
//!  0x00   4     magic           Must be 0x907070b2 (TOBOOT_V2_MAGIC)
//!  0x04   2     reserved_gen    Generation counter (incremented by Toboot on each flash)
//!  0x06   1     start           Starting flash page (16 = 0x4000, after bootloader)
//!  0x07   1     config          Bitmask flags:
//!                                  bit 0: ENABLE_IRQ  - leave IRQs enabled before jump
//!                                  bit 1: AUTORUN     - enter DFU on every cold boot
//!  0x08   4     lock_entry      Set to 0x18349420 to disable manual DFU entry via
//!                                pad shorting (use with caution - can lock you out)
//!  0x0C   4     erase_mask_lo   Bitmask of flash pages 0-31 to erase on update
//!  0x10   4     erase_mask_hi   Bitmask of flash pages 32-63 to erase on update
//!  0x14   4     reserved_hash   XXH32 hash of bytes 0x00-0x13 with seed 0x037a5cf1
//! ```
//!
//! Toboot rewrites `reserved_gen` (incrementing from the previous value) and
//! recomputes `reserved_hash` during each DFU download. The hash is validated
//! on every boot; configs with invalid hashes are ignored.
//!
//! # Boot sequence
//!
//! On reset, Toboot runs the following checks **in order**. Each matching
//! condition overwrites `bootloader_reason`, so the **last** match wins:
//!
//! | Check | Reason code | USB name suffix | Condition |
//! |-------|-------------|-----------------|-----------|
//! | 1 | `COLD_BOOT_CONFIGURATION_FLAG` | `(1)` | Power-on reset + AUTORUN flag set |
//! | 2 | `BOOT_TOKEN_PRESENT` | `(2)` | Magic 0x74624346 in RAM at boot |
//! | 3 | `BUTTON_HELD_DOWN` | `(3)` | Outer capacitive touch pads shorted |
//! | 4 | `BOOT_FAILED_TOO_MANY_TIMES` | `(4)` | `boot_count >= 3` in RAM (app didn't clear it) |
//! | 5 | `NO_PROGRAM_PRESENT` | `(5)` | SP or reset vector out of valid range |
//!
//! If none match, Toboot jumps to the application at `1024 * config.start`.
//!
//! The reason code is visible in the USB device name, e.g.
//! `"Tomu Bootloader (5) v2.0-rc7"`.
//!
//! **Important**: because the last matching check wins, a pad short (reason 3)
//! can be masked by an invalid-app check (reason 5) if the flash is also
//! empty or corrupted.
//!
//! # Application validity check (reason 5)
//!
//! `test_application_invalid()` reads the first two words at the app base
//! address (`1024 * config.start`):
//!
//! - **`app_vectors[0]`** (initial SP) must satisfy
//!   `__ram_start__ <= SP <= __ram_end__` (0x2000_0000..0x2000_2000 for 8 KiB RAM)
//! - **`app_vectors[1]`** (reset vector) must satisfy
//!   `__bl_end__ <= RV < __app_end__` (0x4000..0x10000 for 48 KiB app flash)
//!
//! If either fails, the app is considered absent.
//!
//! # DFU write process
//!
//! When Toboot receives a DFU download:
//!
//! 1. The first 1024-byte block is buffered.
//! 2. `address_for_block(0)` inspects offset 0x94 in the buffer:
//!    - V2 magic found → `version = 2`, address = `config.start * 1024`
//!    - V1 magic found → `version = 1`, legacy address
//!    - Neither → `version = 0`, default address
//! 3. For V2: `reserved_gen` is incremented, the `FAKE` flag is cleared,
//!    and `tb_sign_config()` recomputes the XXH32 hash.
//! 4. Flash pages are erased and programmed word-by-word via the MSC peripheral.
//!
//! # Known issues with Toboot v2.0-rc7
//!
//! - **GPIO pin state not reset**: Toboot leaves GPIO pins (notably PC1 for
//!   capsense) in their configured state. Applications should explicitly reset
//!   pins they need: `gpio.pc_model().modify(|_, w| w.mode1().disabled())`.
//! - **LEDs left on**: The green LED (PA0) may be left driven low. Set DOUT
//!   high before configuring pin mode to avoid a visible glitch.
//!
//! # Diagnostics
//!
//! If your app doesn't boot after flashing:
//!
//! 1. Check the USB device name for the reason code in parentheses.
//! 2. **(3) BUTTON_HELD_DOWN**: The outer pads are being shorted — by a metal
//!    surface, USB hub housing, or solder bridge. Remove the contact.
//! 3. **(4) BOOT_FAILED_TOO_MANY_TIMES**: The app started 3+ times without
//!    clearing `boot_count` at RAM address 0x2000_0000 offset 4. Write 0 to
//!    that byte early in main.
//! 4. **(5) NO_PROGRAM_PRESENT**: Flash is empty or the vector table is invalid.
//!    Verify the binary has a valid SP and reset vector, and that the Toboot
//!    config is at offset 0x94 with a valid XXH32 hash.
//! 5. Note that reason 5 can mask reason 3 (pad short) since it is checked last.
//! 6. Verify the `.toboot` section at 0x4094 with:
//!    `arm-none-eabi-objdump -h target/thumbv6m-none-eabi/release/<demo>`

/// Place this macro invocation at the top level of every demo crate
/// to emit the Toboot V2 configuration header.
///
/// The default invocation uses `config = 0` (IRQs disabled, no autorun):
///
/// ```rust,ignore
/// tomu::toboot_config!();
/// ```
///
/// To set config flags (e.g. enable IRQs):
///
/// ```rust,ignore
/// tomu::toboot_config!(0x01); // ENABLE_IRQ
/// ```
#[macro_export]
macro_rules! toboot_config {
    () => {
        tomu::toboot_config!(0);
    };
    ($config:expr) => {
        #[used]
        #[link_section = ".toboot"]
        #[no_mangle]
        static __TOBOOT_CONFIG: [u8; 24] = {
            let config: u8 = $config;
            [
                0xB2, 0x70, 0x70, 0x90, // magic: 0x907070b2 (little-endian)
                0x00, 0x00,   // reserved_gen: 0 (u16, Toboot increments this)
                0x10,   // start: 16 (= 16 KiB, after Toboot)
                config, // config flags
                0x00, 0x00, 0x00, 0x00, // lock_entry: 0
                0x00, 0x00, 0xFF, 0xFF, // erase_mask_lo: 0xFFFF0000 (pages 16-31)
                0xFF, 0xFF, 0x00, 0x00, // erase_mask_hi: 0x0000FFFF (pages 32-47)
                // reserved_hash: XXH32 of first 20 bytes with seed 0x037a5cf1
                // Pre-computed for gen=0, start=16, config=0, erase=all app pages
                0x78, 0xC4, 0xEC, 0x24,
            ]
        };
    };
}

const TOBOOT_HASH_SEED: u32 = 0x037a_5cf1;
const TOBOOT_CONFIG_ADDR: *const u8 = 0x4094 as *const u8;

/// XXH32 over `data` with the given `seed`. Minimal no_std implementation.
fn xxh32(data: &[u8], seed: u32) -> u32 {
    const P1: u32 = 0x9E37_79B1;
    const P2: u32 = 0x85EB_CA77;
    const P3: u32 = 0xC2B2_AE3D;
    const P4: u32 = 0x27D4_EB2F;
    const P5: u32 = 0x1656_67B1;

    let len = data.len();
    let mut idx = 0;

    let mut h: u32 = if len >= 16 {
        let mut v1 = seed.wrapping_add(P1).wrapping_add(P2);
        let mut v2 = seed.wrapping_add(P2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(P1);
        while idx <= len - 16 {
            v1 = round(v1, read32(data, idx));
            idx += 4;
            v2 = round(v2, read32(data, idx));
            idx += 4;
            v3 = round(v3, read32(data, idx));
            idx += 4;
            v4 = round(v4, read32(data, idx));
            idx += 4;
        }
        v1.rotate_left(1)
            .wrapping_add(v2.rotate_left(7))
            .wrapping_add(v3.rotate_left(12))
            .wrapping_add(v4.rotate_left(18))
    } else {
        seed.wrapping_add(P5)
    };

    h = h.wrapping_add(len as u32);

    while idx <= len - 4 {
        h = h.wrapping_add(read32(data, idx).wrapping_mul(P3));
        h = h.rotate_left(17).wrapping_mul(P4);
        idx += 4;
    }
    while idx < len {
        h = h.wrapping_add((data[idx] as u32).wrapping_mul(P5));
        h = h.rotate_left(11).wrapping_mul(P1);
        idx += 1;
    }

    h ^= h >> 15;
    h = h.wrapping_mul(P2);
    h ^= h >> 13;
    h = h.wrapping_mul(P3);
    h ^= h >> 16;
    h
}

fn round(acc: u32, input: u32) -> u32 {
    acc.wrapping_add(input.wrapping_mul(0x85EB_CA77))
        .rotate_left(13)
        .wrapping_mul(0x9E37_79B1)
}

fn read32(data: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]])
}

/// Read the Toboot V2 config from flash at 0x4094, recompute the XXH32 hash
/// over the first 20 bytes, and assert it matches `reserved_hash`.
///
/// Call this early in `main()` to detect flash corruption or Toboot signing
/// issues. On mismatch, panics (which halts the CPU via `panic_halt`).
///
/// # Safety
///
/// Reads 24 bytes from a fixed flash address. Safe on EFM32HG with a valid
/// Toboot image at 0x4000.
pub fn verify_config() {
    let cfg = unsafe { core::slice::from_raw_parts(TOBOOT_CONFIG_ADDR, 24) };
    let computed = xxh32(&cfg[..20], TOBOOT_HASH_SEED);
    let stored = read32(cfg, 20);
    assert_eq!(
        computed, stored,
        "Toboot config hash mismatch: computed 0x{:08X}, stored 0x{:08X}",
        computed, stored,
    );
}
