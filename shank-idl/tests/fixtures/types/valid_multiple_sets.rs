#[derive(BorshSerialize)]
pub struct OneHashSetStruct {
    pub u8_set: HashSet<u8>,
}

#[derive(BorshSerialize)]
pub struct MultipleHashSetsStruct {
    pub u8_set: HashSet<u8>,
    pub string_set: HashSet<String>,
    pub option_i128_set: HashSet<Option<i128>>,
    pub vec_custom_set: HashSet<Vec<Custom>>,
}

#[derive(BorshSerialize)]
pub struct OneBTreeSetStruct {
    pub u8_set: BTreeSet<u8>,
}

#[derive(BorshSerialize)]
pub struct MultipleBTreeSetsStruct {
    pub u8_set: BTreeSet<u8>,
    pub string_set: BTreeSet<String>,
    pub option_i128_set: BTreeSet<Option<i128>>,
    pub vec_custom_set: BTreeSet<Vec<Custom>>,
}

#[derive(BorshSerialize)]
pub struct NestedSetsStruct {
    pub vec_hash_map_u8: Vec<HashSet<u8>>,
    pub option_btree_map_u8: Option<BTreeSet<u8>>,
}
