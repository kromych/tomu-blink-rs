#!/bin/bash

# Check if file is provided
if [ "$#" -lt 2 ]; then
    echo "Usage: $0 <elf_file>"
    exit 1
fi

ELF_FILE="$1"
DIR_NAME=$(dirname "$ELF_FILE")
BASE_NAME=$(basename "$ELF_FILE")
BIN_FILE="$DIR_NAME/$BASE_NAME.bin"
DFU_FILE="$DIR_NAME/$BASE_NAME.dfu"

coreutils cp "$ELF_FILE" "$BIN_FILE"
# Convert ELF to BIN
cargo objcopy --release -p "$BASE_NAME" -- -O binary "$BIN_FILE"

coreutils cp "$BIN_FILE" "$DFU_FILE"
# Add DFU suffix
dfu-suffix -v 1209 -p 70b1 -a "$DFU_FILE" > /dev/null
coreutils echo "DFU file generated as $DFU_FILE"

dfu-util -D "$DFU_FILE" > /dev/null
