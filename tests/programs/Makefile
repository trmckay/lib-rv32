PROJECT = prog
SRC = .
BUILD = .

C_FILES = $(wildcard $(SRC)/*.c)
AS_FILES = $(wildcard $(SRC)/*.s)

C_OBJ_FILES = $(patsubst $(SRC)/%.c,$(BUILD)/%.o,$(C_FILES))
AS_OBJ_FILES = $(patsubst $(SRC)/%.s,$(BUILD)/%.o,$(AS_FILES))

LINKER_SCRIPT = ../link.ld

RISCV_PREFIX = riscv64-unknown-elf-
CC = $(RISCV_PREFIX)gcc
LD = $(RISCV_PREFIX)ld
OBJCOPY = $(RISCV_PREFIX)objcopy
OBJDUMP = $(RISCV_PREFIX)objdump

TARGET_ARCH = -march=rv32i -mabi=ilp32
CCFLAGS = $(TARGET_ARCH) -O0 -mno-relax -nostdlib -nostartfiles -mcmodel=medany -g

all: dirs $(BUILD)/$(PROJECT).bin

$(BUILD)/%.o: $(SRC)/%.c
	$(CC) -c -o $@ $< $(CCFLAGS)

$(BUILD)/%.o: $(SRC)/%.s
	$(CC) -c -o $@ $< $(CCFLAGS)

$(BUILD)/$(PROJECT).elf: ${AS_OBJ_FILES} ${C_OBJ_FILES}
	$(CC) -o $@ $^ -T $(LINKER_SCRIPT) $(CCFLAGS)

$(BUILD)/$(PROJECT).bin: $(BUILD)/$(PROJECT).elf
	$(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@
	rm -f $(C_OBJ_FILES) $(AS_OBJ_FILES) $(BUILD)/$(PROJECT).elf

dirs:
	mkdir -p $(BUILD)

clean:
	rm -rf $(C_OBJ_FILES) $(AS_OBJ_FILES) $(BUILD)/$(PROJECT).bin
