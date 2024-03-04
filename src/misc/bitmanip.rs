pub fn low_bit(num: usize) -> usize {
    return num & (!num + 1);
}
