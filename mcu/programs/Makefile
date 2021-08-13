BUILD = .
SRC = .

# source files
C_FILES := $(wildcard $(SRC)/*.c)
AS_FILES := $(wildcard $(SRC)/*.s)
ASP_FILES := $(wildcard $(SRC)/*.S)

# object files
C_OBJ_FILES := $(patsubst $(SRC)/%.c,$(BUILD)/%.o,$(C_FILES))
AS_OBJ_FILES := $(patsubst $(SRC)/%.s,$(BUILD)/%.o,$(AS_FILES))
ASP_OBJ_FILES := $(patsubst $(SRC)/%.S,$(BUILD)/%.o,$(ASP_FILES))

LINKER_SCRIPT = ../link.ld

USE_RV64 := $(shell command -v riscv64-unknown-elf-gcc 2> /dev/null)
ifdef USE_RV64
RISCV_PREFIX = riscv64-unknown-elf-
else
RISCV_PREFIX = riscv32-unknown-elf-
endif

CC = $(RISCV_PREFIX)gcc
LD = $(RISCV_PREFIX)ld
OBJCOPY = $(RISCV_PREFIX)objcopy
OBJDUMP = $(RISCV_PREFIX)objdump

TARGET_ARCH = -march=rv32i -mabi=ilp32
CCFLAGS = -O0 $(TARGET_ARCH) -mno-relax -nostdlib -nostartfiles -mcmodel=medany

all: $(BUILD)/prog.bin $(BUILD)/dump.txt cleanup

$(BUILD)/%.o: $(SRC)/%.c
	$(CC) -c -o $@ $< $(CCFLAGS)

$(BUILD)/%.o: $(SRC)/%.s
	$(CC) -c -o $@ $< $(CCFLAGS)

$(BUILD)/%.o: $(SRC)/%.S
	$(CC) -c -o $@ $< $(CCFLAGS)

$(BUILD)/prog.elf: ${AS_OBJ_FILES} ${C_OBJ_FILES} ${ASP_OBJ_FILES}
	$(CC) -o $@ $^ -T $(LINKER_SCRIPT) $(CCFLAGS)

$(BUILD)/dump.txt: $(BUILD)/prog.elf
	$(OBJDUMP) -S $< > $@

$(BUILD)/prog.bin: $(BUILD)/prog.elf
	$(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@

cleanup:
	rm -rf $(C_OBJ_FILES) $(ASP_OBJ_FILES) $(AS_OBJ_FILES) $(BUILD)/prog.elf

print-%  : ; @echo $* = $($*)
