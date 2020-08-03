TARGET      := riscv64imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/os
BIN_FILE    := target/$(TARGET)/$(MODE)/kernel.bin

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

.PHONY: doc kernel build clean qemu run

# default build an output binary file
build: $(BIN_FILE)

# use comment in rust file to generate os document
doc:
	@cargo doc --document-private-items
       
# compile kernel
kernel:
	@cargo build
      
# generate kernel binary file
${BIN_FILE}: kernel
	@$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $@

# check disassembly result
asm:
	@$(OBJDUMP) -d $(KERNEL_FILE) | less
       
# clean output compiled file
clean:
	@cargo clean
       
# run qemu
qemu: build
	@qemu-system-riscv64 \
      -machine virt \
      -nographic \
      -bios default \
      -device loader,file=$(BIN_FILE),addr=0x80200000

# one-click operation
run: build qemu