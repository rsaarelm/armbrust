GDB=arm-none-eabi-gdb
QEMU=/opt/gnuarmeclipse/qemu/bin/qemu-system-gnuarmeclipse
QEMUFLAGS := -M STM32F4-Discovery -m 32M -nographic

TRIPLE=thumbv7em-none-eabi
TARGET=target/$(TRIPLE)/debug/arm-test

$(TARGET):
	cargo build

.PHONY: $(TARGET)

arm-test.bin: $(TARGET)
	arm-none-eabi-objcopy -O binary $< $@

run: $(TARGET)
	$(QEMU) $(QEMUFLAGS) -kernel $<

# Start qemu for debugging.
debug: $(TARGET)
	@echo
	@echo "Starting QEMU for debugging"
	@echo "Connect using:"
	@echo "    arm-none-eabi-gdb -ex 'target remote localhost:1234'"
	@echo
	$(QEMU) -s -S $(QEMUFLAGS) -kernel $<
