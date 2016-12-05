TRIPLE=thumbv7em-none-eabi
APP=arm-test
GDB=arm-none-eabi-gdb
QEMU=qemu-system-arm
QEMUFLAGS := -M versatilepb -m 32M -nographic

all: $(APP).bin

$(APP).bin: target/$(TRIPLE)/debug/$(APP)
	arm-none-eabi-objcopy -O binary $< $@

target/$(TRIPLE)/debug/$(APP):
	cargo build

run: target/$(TRIPLE)/debug/$(APP)
	$(QEMU) $(QEMUFLAGS) -kernel $<

# Start qemu for debugging.
start: target/$(TRIPLE)/debug/$(APP)
	$(QEMU) $(QEMUFLAGS) -kernel $< -s -S

# Debug with:
#     arm-none-eabi-gdb -ex "target remote localhost:1234"
