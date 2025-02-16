pub fn swap_half_word_byte_sequence_u32(value: u32) -> u32 {
    // 将每个16位半字互换
    ((value & 0x0000FFFF) << 16) | ((value & 0xFFFF0000) >> 16)
}