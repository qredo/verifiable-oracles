use super::{CodeBody, Felt, Instruction, Node, ProcedureId, RpoDigest, ToString};
use crate::MAX_PUSH_INPUTS;
use num_enum::TryFromPrimitive;

use vm_core::utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

mod deserialization;
mod serialization;

// OPERATION CODES ENUM
// ================================================================================================

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum OpCode {
    Assert = 0,
    AssertEq = 1,
    AssertEqw = 2,
    Assertz = 3,
    Add = 4,
    AddImm = 5,
    Sub = 6,
    SubImm = 7,
    Mul = 8,
    MulImm = 9,
    Div = 10,
    DivImm = 11,
    Neg = 12,
    Inv = 13,
    Incr = 14,
    Pow2 = 15,
    Exp = 16,
    ExpImm = 17,
    ExpBitLength = 18,
    Not = 19,
    And = 20,
    Or = 21,
    Xor = 22,
    Eq = 23,
    EqImm = 24,
    Neq = 25,
    NeqImm = 26,
    Eqw = 27,
    Lt = 28,
    Lte = 29,
    Gt = 30,
    Gte = 31,
    IsOdd = 32,

    // ----- ext2 operations ----------------------------------------------------------------------
    Ext2Add = 33,
    Ext2Sub = 34,
    Ext2Mul = 35,
    Ext2Div = 36,
    Ext2Neg = 37,
    Ext2Inv = 38,

    // ----- u32 manipulation ---------------------------------------------------------------------
    U32Test = 39,
    U32TestW = 40,
    U32Assert = 41,
    U32Assert2 = 42,
    U32AssertW = 43,
    U32Split = 44,
    U32Cast = 45,
    U32CheckedAdd = 46,
    U32CheckedAddImm = 47,
    U32WrappingAdd = 48,
    U32WrappingAddImm = 49,
    U32OverflowingAdd = 50,
    U32OverflowingAddImm = 51,
    U32OverflowingAdd3 = 52,
    U32WrappingAdd3 = 53,
    U32CheckedSub = 54,
    U32CheckedSubImm = 55,
    U32WrappingSub = 56,
    U32WrappingSubImm = 57,
    U32OverflowingSub = 58,
    U32OverflowingSubImm = 59,
    U32CheckedMul = 60,
    U32CheckedMulImm = 61,
    U32WrappingMul = 62,
    U32WrappingMulImm = 63,
    U32OverflowingMul = 64,
    U32OverflowingMulImm = 65,
    U32OverflowingMadd = 66,
    U32WrappingMadd = 67,
    U32CheckedDiv = 68,
    U32CheckedDivImm = 69,
    U32UncheckedDiv = 70,
    U32UncheckedDivImm = 71,
    U32CheckedMod = 72,
    U32CheckedModImm = 73,
    U32UncheckedMod = 74,
    U32UncheckedModImm = 75,
    U32CheckedDivMod = 76,
    U32CheckedDivModImm = 77,
    U32UncheckedDivMod = 78,
    U32UncheckedDivModImm = 79,
    U32CheckedAnd = 80,
    U32CheckedOr = 81,
    U32CheckedXor = 82,
    U32CheckedNot = 83,
    U32CheckedShr = 84,
    U32CheckedShrImm = 85,
    U32UncheckedShr = 86,
    U32UncheckedShrImm = 87,
    U32CheckedShl = 88,
    U32CheckedShlImm = 89,
    U32UncheckedShl = 90,
    U32UncheckedShlImm = 91,
    U32CheckedRotr = 92,
    U32CheckedRotrImm = 93,
    U32UncheckedRotr = 94,
    U32UncheckedRotrImm = 95,
    U32CheckedRotl = 96,
    U32CheckedRotlImm = 97,
    U32UncheckedRotl = 98,
    U32UncheckedRotlImm = 99,
    U32CheckedPopcnt = 100,
    U32UncheckedPopcnt = 101,
    U32CheckedEq = 102,
    U32CheckedEqImm = 103,
    U32CheckedNeq = 104,
    U32CheckedNeqImm = 105,
    U32CheckedLt = 106,
    U32UncheckedLt = 107,
    U32CheckedLte = 108,
    U32UncheckedLte = 109,
    U32CheckedGt = 110,
    U32UncheckedGt = 111,
    U32CheckedGte = 112,
    U32UncheckedGte = 113,
    U32CheckedMin = 114,
    U32UncheckedMin = 115,
    U32CheckedMax = 116,
    U32UncheckedMax = 117,

    // ----- stack manipulation -------------------------------------------------------------------
    Drop = 118,
    DropW = 119,
    PadW = 120,
    Dup0 = 121,
    Dup1 = 122,
    Dup2 = 123,
    Dup3 = 124,
    Dup4 = 125,
    Dup5 = 126,
    Dup6 = 127,
    Dup7 = 128,
    Dup8 = 129,
    Dup9 = 130,
    Dup10 = 131,
    Dup11 = 132,
    Dup12 = 133,
    Dup13 = 134,
    Dup14 = 135,
    Dup15 = 136,
    DupW0 = 137,
    DupW1 = 138,
    DupW2 = 139,
    DupW3 = 140,
    Swap1 = 141,
    Swap2 = 142,
    Swap3 = 143,
    Swap4 = 144,
    Swap5 = 145,
    Swap6 = 146,
    Swap7 = 147,
    Swap8 = 148,
    Swap9 = 149,
    Swap10 = 150,
    Swap11 = 151,
    Swap12 = 152,
    Swap13 = 153,
    Swap14 = 154,
    Swap15 = 155,
    SwapW1 = 156,
    SwapW2 = 157,
    SwapW3 = 158,
    SwapDW = 159,
    MovUp2 = 160,
    MovUp3 = 161,
    MovUp4 = 162,
    MovUp5 = 163,
    MovUp6 = 164,
    MovUp7 = 165,
    MovUp8 = 166,
    MovUp9 = 167,
    MovUp10 = 168,
    MovUp11 = 169,
    MovUp12 = 170,
    MovUp13 = 171,
    MovUp14 = 172,
    MovUp15 = 173,
    MovUpW2 = 174,
    MovUpW3 = 175,
    MovDn2 = 176,
    MovDn3 = 177,
    MovDn4 = 178,
    MovDn5 = 179,
    MovDn6 = 180,
    MovDn7 = 181,
    MovDn8 = 182,
    MovDn9 = 183,
    MovDn10 = 184,
    MovDn11 = 185,
    MovDn12 = 186,
    MovDn13 = 187,
    MovDn14 = 188,
    MovDn15 = 189,
    MovDnW2 = 190,
    MovDnW3 = 191,
    CSwap = 192,
    CSwapW = 193,
    CDrop = 194,
    CDropW = 195,

    // ----- input / output operations ------------------------------------------------------------
    PushU8 = 196,
    PushU16 = 197,
    PushU32 = 198,
    PushFelt = 199,
    PushWord = 200,
    PushU8List = 201,
    PushU16List = 202,
    PushU32List = 203,
    PushFeltList = 204,

    Locaddr = 205,
    Sdepth = 206,
    Caller = 207,
    Clk = 208,

    MemLoad = 209,
    MemLoadImm = 210,
    MemLoadW = 211,
    MemLoadWImm = 212,
    LocLoad = 213,
    LocLoadW = 214,
    MemStore = 215,
    MemStoreImm = 216,
    LocStore = 217,
    MemStoreW = 218,
    MemStoreWImm = 219,
    LocStoreW = 220,

    MemStream = 221,
    AdvPipe = 222,

    AdvPush = 223,
    AdvLoadW = 224,

    AdvInject = 225,

    // ----- cryptographic operations -------------------------------------------------------------
    Hash = 226,
    HMerge = 227,
    HPerm = 228,
    MTreeGet = 229,
    MTreeSet = 230,
    MTreeMerge = 231,
    MTreeVerify = 232,

    // ----- STARK proof verification -------------------------------------------------------------
    FriExt2Fold4 = 233,

    // ----- exec / call --------------------------------------------------------------------------
    ExecLocal = 234,
    ExecImported = 235,
    CallLocal = 236,
    CallMastRoot = 237,
    CallImported = 238,
    SysCall = 239,

    // ----- control flow -------------------------------------------------------------------------
    IfElse = 253,
    Repeat = 254,
    While = 255,
}

impl Serializable for OpCode {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(*self as u8);
    }
}

impl Deserializable for OpCode {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u8()?;
        Self::try_from(value).map_err(|_| {
            DeserializationError::InvalidValue("could not read a valid opcode".to_string())
        })
    }
}
