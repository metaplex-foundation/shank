#[derive(BorshSerialize)]
pub struct TwoElementTuples {
    pub u8_u8: (u8, u8),
    pub u8_u16: (u8, u16),
    pub string_custom: (String, Custom),
}

#[derive(BorshSerialize)]
pub struct NestedTwoElementTuples {
    pub vec_u8_u8: Vec<(u8, u8)>,
    pub hash_map_u8_u16_string_custom: HashMap<(u8, u16), (String, Custom)>,
}
