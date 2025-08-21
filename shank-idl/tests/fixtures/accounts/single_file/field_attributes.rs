use shank::ShankAccount;

#[derive(ShankAccount)]
pub struct FieldAttributesExample {
    pub normal_field: u64,

    #[idl_name("customName")]
    pub renamed_field: String,

    #[skip]
    pub skipped_field: bool,

    #[idl_name("renamedAndPadded")]
    #[padding]
    pub renamed_padding_field: [u8; 32],

    #[idl_type("u32")]
    #[idl_name("customTypedField")]
    pub custom_typed_field: SomeWrapper<u32>,

    #[skip]
    pub slice_field: &[u8],
    
    #[skip]
    pub complex_skipped_field: Box<dyn std::any::Any + Send + Sync>,
}

pub struct SomeWrapper<T>(pub T);
