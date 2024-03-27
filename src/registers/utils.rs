pub const fn truncate_to(value: u32, num_bits: u32) -> u32 {
    let shift_by = u32::BITS - num_bits;
    (value << shift_by) >> shift_by
}
