#[derive(BorshSerialize)]
pub struct OneHashMapStruct {
    pub u8_u8_map: HashMap<u8, u8>,
}

#[derive(BorshSerialize)]
pub struct MultipleHashMapsStruct {
    pub u8_string_map: HashMap<u8, String>,
    pub string_option_i128_map: HashMap<String, Option<i128>>,
    pub option_string_vec_custom_map: HashMap<Option<String>, Vec<Custom>>,
}

#[derive(BorshSerialize)]
pub struct OneBTreeMapStruct {
    pub u8_u8_map: BTreeMap<u8, u8>,
}

#[derive(BorshSerialize)]
pub struct MultipleMapsStruct {
    pub u8_string_btree_map: BTreeMap<u8, String>,
    pub option_string_vec_custom_btree_map:
        BTreeMap<Option<String>, Vec<Custom>>,
    pub i16_option_bool_hash_map: HashMap<i16, Option<bool>>,
}

#[derive(BorshSerialize)]
pub struct NestedMapsStruct {
    pub vec_hash_map_u8_u8: Vec<HashMap<u8, u8>>,
    pub option_btree_map_u8_u8: Option<BTreeMap<u8, u8>>,
}
