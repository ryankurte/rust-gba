
APP=gba-example
TARGET=arm7tdmi
OUTDIR=target/$(TARGET)/debug
VBA=vba

all: build size fix

build: $(OUTDIR)/$(APP)

# Target path required due to https://github.com/rust-lang/cargo/issues/4905
$(OUTDIR)/$(APP):
	RUST_TARGET_PATH=`pwd` cargo xbuild --release --target $(TARGET).json 

fix: $(OUTDIR)/$(APP).gba

$(OUTDIR)/$(APP).gba: build
	arm-none-eabi-objcopy -v -O binary $(OUTDIR)/$(APP) $(OUTDIR)/$(APP).gba
	gbafix $(OUTDIR)/$(APP).gba

size: build
	arm-none-eabi-size $(OUTDIR)/$(APP)

dump: build
	arm-none-eabi-objdump -CDS --architecture=arm7tdmi $(OUTDIR)/$(APP)

nm: build
	arm-none-eabi-nm -C $(OUTDIR)/$(APP)

run: build fix
	$(VBA) $(OUTDIR)/$(APP).gba

debug:
	arm-none-eabi-gdb $(OUTDIR)/$(APP) --tui --eval-command="tar rem :55555"

file:
	file $(OUTDIR)/$(APP)

clean:
	rm -rf target/

.PHONY: build size dump nm run $(OUTDIR)/$(APP) $(OUTDIR)/$(APP).gba
