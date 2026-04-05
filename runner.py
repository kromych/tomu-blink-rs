#!/usr/bin/env python3
"""Cargo runner for Tomu: ELF -> DFU flash via dfu-util."""

import shutil
import subprocess
import sys
from pathlib import Path


def find_llvm_objcopy() -> Path:
    """Locate llvm-objcopy from the rustup llvm-tools component."""
    sysroot = subprocess.check_output(
        ["rustc", "--print", "sysroot"], text=True
    ).strip()
    verbose = subprocess.check_output(["rustc", "-vV"], text=True)
    host = ""
    for line in verbose.splitlines():
        if line.startswith("host: "):
            host = line[len("host: "):]
            break
    if not host:
        sys.exit("error: could not determine rustc host triple")
    objcopy = Path(sysroot) / "lib" / "rustlib" / host / "bin" / "llvm-objcopy"
    if sys.platform == "win32" and not objcopy.suffix:
        objcopy = objcopy.with_suffix(".exe")
    if not objcopy.exists():
        sys.exit(
            f"error: llvm-objcopy not found at {objcopy}\n"
            "  install it with: rustup component add llvm-tools"
        )
    return objcopy


def main():
    if len(sys.argv) < 2:
        sys.exit(f"Usage: {sys.argv[0]} <elf_file>")

    elf_file = Path(sys.argv[1])
    bin_file = elf_file.with_suffix(elf_file.suffix + ".bin")
    dfu_file = elf_file.with_suffix(elf_file.suffix + ".dfu")

    objcopy = find_llvm_objcopy()

    # Convert ELF to raw binary.
    subprocess.check_call([str(objcopy), "-O", "binary", str(elf_file), str(bin_file)])

    # Create DFU image with Tomu VID/PID suffix.
    shutil.copy2(str(bin_file), str(dfu_file))
    subprocess.check_call(
        ["dfu-suffix", "-v", "1209", "-p", "70b1", "-a", str(dfu_file)],
        stdout=subprocess.DEVNULL,
    )

    # Flash. dfu-util may return non-zero (e.g. 74) when the device
    # reboots itself after a successful download — treat that as success.
    print(f"Flashing {dfu_file} ...")
    subprocess.run(["dfu-util", "-D", str(dfu_file)])


if __name__ == "__main__":
    main()
