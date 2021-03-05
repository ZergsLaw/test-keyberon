
build:
	cargo build --release
	cargo objcopy --bin stm32f103rbt6 --target thumbv7m-none-eabi --release -- -O binary stm32f103rbt6.bin

write:
	st-flash erase 
	st-flash write stm32f103rbt6.bin 0x8000000

run: build write