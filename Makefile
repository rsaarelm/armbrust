TARGET=target/thumbv7m-none-eabi/debug/arm-test

QEMU_ARGS=-cpu cortex-m3 -machine lm3s6965evb -nographic -monitor null

.PHONY: $(TARGET)

$(TARGET):
	xargo build --target thumbv7m-none-eabi

# Dump asm of target.
dump: $(TARGET)
	arm-none-eabi-objdump --demangle --disassemble $(TARGET)

# Get size stats of target
stats: $(TARGET)
	arm-none-eabi-readelf -h $(TARGET)
	arm-none-eabi-size $(TARGET)

# Run in QEMU.
run: $(TARGET)
	qemu-system-arm $(QEMU_ARGS) -kernel $(TARGET)

# Start QEMU in paused state, connect with debugger
start: $(TARGET)
	@echo
	@echo Starting QEMU for debugging
	@echo You can now do 'make debug' in a separate terminal
	@echo
	qemu-system-arm $(QEMU_ARGS) -S -s -kernel $(TARGET)

debug:
	@echo
	@echo Starting debugger
	@echo "This will only work if you're already running 'make start'"
	@echo
	arm-none-eabi-gdb -q $(TARGET) -ex 'target remote :1234'
