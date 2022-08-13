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

#[derive(BorshSerialize)]
pub struct MoreElementTuples {
    pub u8_u8_u8: (u8, u8, u8),
    pub u8_u16_i32_bool: (u8, u16, i32, bool),
    pub string_custom_option_i128_u8_u16_u32_u64:
        (String, Custom, Option<i128>, u8, u16, u32, u64),
}

#[derive(BorshSerialize)]
pub struct NestedMultiElementTuples {
    pub vec_u8_u8_u16_u16_u32_uy32: Vec<(u8, u8, u16, u16, u32, u32)>,
    pub hash_map_u8_u16_u32_string_custom_u8_u8:
        HashMap<(u8, u16, u32), (String, Custom, u8, u8)>,
}
