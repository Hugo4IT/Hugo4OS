work-dir := `pwd`

cargo-args := "--target targets/x86_64-hugo4os.json -Zbuild-std=core,compiler_builtins,alloc -Zbuild-std-features=compiler-builtins-mem"
qemu-args := "--no-reboot -s -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio -cpu max,+sse -m 4096 -accel tcg,tb-size=1024"

@check-dependencies:
	printf 'Checking dependencies...     '
	which 'llvm-objcopy' >/dev/null || ( echo 'Missing llvm-objcopy (llvm-tools), please install llvm-tools: `rustup component add llvm-tools-preview`'; false )
	which 'llvm-ar' >/dev/null || ( echo 'Missing llvm-ar (llvm-tools), this has been available since rust nightly-2019-03-29. Run `rustup upgrade` to update your toolchains'; false )
	echo 'Ok!'

@build-bios: check-dependencies
	printf 'Building BIOS bootloader...  '
	llvm-objcopy --strip-debug {{work-dir}}/target/x86_64-hugo4os/release/hugo4os {{work-dir}}/target/x86_64-hugo4os/release/kernel_stripped-hugo4os
	cd {{work-dir}}/target/x86_64-hugo4os/release && llvm-objcopy -I binary -O elf64-x86-64 --binary-architecture=i386:x86-64 --rename-section '.data=.kernel' --redefine-sym '_binary_kernel_stripped_hugo4os_start=_kernel_start_addr' --redefine-sym '_binary_kernel_stripped_hugo4os_end=_kernel_end_addr' --redefine-sym '_binary_kernel_stripped_hugo4os_size=_kernel_size' kernel_stripped-hugo4os {{work-dir}}/target/x86_64-hugo4os/release/kernel_bin-hugo4os.o
	llvm-ar crs {{work-dir}}/target/x86_64-hugo4os/release/libkernel_bin-hugo4os.a {{work-dir}}/target/x86_64-hugo4os/release/kernel_bin-hugo4os.o
	cd crates/bootloader_bios && KERNEL='{{work-dir}}/target/x86_64-hugo4os/release/hugo4os' RUSTFLAGS='-C opt-level=s -C strip=debuginfo -L native={{work-dir}}/target/x86_64-hugo4os/release -l static=kernel_bin-hugo4os' cargo build --bin bios --release -Zunstable-options --target x86_64-bootloader.json -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem --quiet
	echo 'Ok!'

@build-uefi:
	:

@build-kernel +ARGS:
	printf 'Building kernel...           '
	RUSTFLAGS='-C strip=debuginfo' cargo build {{ARGS}} {{cargo-args}} --release --quiet
	echo 'Ok!'

@create-image-bios +ARGS: (build-kernel ARGS) build-bios
	printf 'Creating bootable image...   '
	llvm-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 {{work-dir}}/target/x86_64-bootloader/release/bios {{work-dir}}/img/hugo4os-bios.img
	echo 'Ok!'

@run *ARGS: (create-image-bios ARGS)
	printf 'Running Hugo4OS with Qemu... '
	qemu-system-x86_64 -drive format=raw,file=./img/hugo4os-bios.img {{qemu-args}}
	echo 'Did you like it?'