# Make sure to replace "your-app-name" below with the anme of your binary!
APP ?= your-app-name
TARGET ?= arm7tdmi
OUT_DIR ?= target/$(TARGET)/release
VBA ?= vba

$(OUT_DIR)/$(APP):
	RUST_TARGET_PATH=$(shell pwd) cargo xbuild --release --target $(TARGET)

$(OUT_DIR)/$(APP).gba: $(OUT_DIR)/$(APP)
	rust-objcopy -O binary $(OUT_DIR)/$(APP) $(OUT_DIR)/$(APP).gba
	gbafix $(OUT_DIR)/$(APP).gba

clean:
	rm -f $(OUT_DIR)/$(APP) $(OUT_DIR)/$(APP).gba

run: $(OUT_DIR)/$(APP).gba
	$(VBA) $(OUT_DIR)/$(APP).gba

build-cargo: $(OUT_DIR)/$(APP)
build: $(OUT_DIR)/$(APP).gba
all: build

.PHONY: clean build build-cargo run all
