/// Shortcut to declare an instruction as a `const u32`.
macro_rules! declare_ir {
    ($name:ident, $code:expr) => {
        #[allow(dead_code)]
        pub const $name: u32 = $code;
    };
}

// ADD_SAME_REG_FIELDS_IR[i] = "add Xi, Xi, Xi"
pub const ADD_SAME_REG_FIELDS_IRS: [u32; 32] = [
    0x00000033, 0x001080b3, 0x00210133, 0x003181b3, 0x00420233, 0x005282b3, 0x00630333, 0x007383b3,
    0x00840433, 0x009484b3, 0x00a50533, 0x00b585b3, 0x00c60633, 0x00d686b3, 0x00e70733, 0x00f787b3,
    0x01080833, 0x011888b3, 0x01290933, 0x013989b3, 0x014a0a33, 0x015a8ab3, 0x016b0b33, 0x017b8bb3,
    0x018c0c33, 0x019c8cb3, 0x01ad0d33, 0x01bd8db3, 0x01ce0e33, 0x01de8eb3, 0x01ef0f33, 0x01ff8fb3,
];

declare_ir!(ADDI_X0_X0_17, 0x01100013);
declare_ir!(XORI_X5_X6_82, 0x05234293);
declare_ir!(ADDI_X5_X6_NEG_12, 0xff430293);
declare_ir!(ADDI_X5_X6_NEG_1, 0xfff30293);
declare_ir!(ADDI_X5_X6_NEG_2048, 0x80030293);
declare_ir!(ADDI_X5_X6_0, 0x00030293);
declare_ir!(ADDI_X5_X6_2047, 0x7ff30293);
declare_ir!(JAL_X0_NEG_8, 0xff9ff06f);
declare_ir!(JAL_X0_16, 0x0100006f);
declare_ir!(LUI_X5_4, 0x000042b7);
declare_ir!(AUIPC_X5_4, 0x00004297);
declare_ir!(JAL_X5_20, 0x014002ef);
declare_ir!(JALR_X5_X5_4, 0x004282e7);
declare_ir!(BEQ_X5_X5_12, 0x00528663);
declare_ir!(LW_X5_0_X5, 0x0002a283);
declare_ir!(SW_X5_0_X5, 0x0052a023);
declare_ir!(BEQ_X5_X5_80, 0x04528863);
declare_ir!(BNE_X5_X5_76, 0x04529663);
declare_ir!(BLT_X5_X5_72, 0x0452c463);
declare_ir!(BGEU_X5_X5_68, 0x0452f263);
declare_ir!(LB_X5_0_X5, 0x00028283);
declare_ir!(LBU_X5_0_X5, 0x0002c283);
declare_ir!(LH_X5_0_X5, 0x00029283);
declare_ir!(LHU_X5_0_X5, 0x0002d283);
declare_ir!(SB_X5_0_X5, 0x00528023);
declare_ir!(SH_X5_0_X5, 0x00529023);
declare_ir!(ADDI_X5_X5_1, 0x00128293);
declare_ir!(SLLI_X5_X5_1, 0x00129293);
declare_ir!(SLTI_X5_X5_1, 0x0012a293);
declare_ir!(SLTU_X5_X5_X5, 0x0052b2b3);
declare_ir!(XORI_X5_X5_1, 0x0012c293);
declare_ir!(SRAI_X5_X5_1, 0x4012d293);
declare_ir!(ORI_X5_X5_1, 0x0012e293);
declare_ir!(ANDI_X5_X5_1, 0x0012f293);
declare_ir!(SUB_X5_X5_X5, 0x405282b3);
declare_ir!(ADDI_X6_X0_1, 0x00100313);
declare_ir!(SW_X5_16_X5, 0x0052a823);
declare_ir!(SW_X5_NEG_40_X5, 0xfc52ac23);
declare_ir!(SW_A0_NEG_36_SP, 0xfca42e23);
declare_ir!(SW_A0_NEG_20_S0, 0xfea42623);
declare_ir!(JAL_X0_NEG_4, 0xffdff06f);
declare_ir!(BNE_X0_X5_NEG_4, 0xfe501ee3);
