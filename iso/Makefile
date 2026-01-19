# RururuOS ISO Build Makefile

VERSION := 1.0.0
ARCH := x86_64
ISO_NAME := rururu-$(VERSION)-$(ARCH).iso

.PHONY: all clean iso packages check

all: check packages iso

check:
	@echo "Checking build environment..."
	@which mkarchiso > /dev/null || (echo "archiso not installed" && exit 1)
	@which cargo > /dev/null || (echo "cargo not installed" && exit 1)
	@echo "Environment OK"

packages:
	@echo "Building RururuOS packages..."
	cd .. && cargo build --release --workspace

iso: packages
	@echo "Building ISO..."
	sudo ./build.sh full

clean:
	@echo "Cleaning..."
	sudo rm -rf work out
	cd .. && cargo clean

test-qemu:
	@echo "Testing ISO in QEMU..."
	@if [ -f "out/$(ISO_NAME)" ]; then \
		qemu-system-x86_64 \
			-cdrom out/$(ISO_NAME) \
			-m 4G \
			-enable-kvm \
			-cpu host \
			-smp 4 \
			-vga virtio \
			-display gtk; \
	else \
		echo "ISO not found. Run 'make iso' first."; \
	fi

help:
	@echo "RururuOS ISO Build System"
	@echo ""
	@echo "Targets:"
	@echo "  all        - Build everything"
	@echo "  check      - Check build environment"
	@echo "  packages   - Build RururuOS packages"
	@echo "  iso        - Build ISO image"
	@echo "  clean      - Clean build artifacts"
	@echo "  test-qemu  - Test ISO in QEMU"
	@echo "  help       - Show this help"
