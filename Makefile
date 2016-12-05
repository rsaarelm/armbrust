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

debug: target/$(TRIPLE)/debug/$(APP)
	tmux new-session -d -s rust
	tmux new-window -t rust:1 "$(QEMU) $(QEMUFLAGS) -kernel $< -s -S"
	tmux split-window -t rust "$(GDB) -ex 'target remote localhost:1234'"
	tmux a -t rust
	tmux kill-session -t rust
