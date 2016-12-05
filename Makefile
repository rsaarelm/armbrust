TRIPLE=thumbv7em-none-eabi

all: test.bin

test.bin: target/$(TRIPLE)/debug/test
	arm-none-eabi-objcopy -O binary $< $@

target/$(TRIPLE)/debug/test:
	cargo build

run: test.bin
	qemu-system-arm -M versatilepb -nographic -kernel test.bin
