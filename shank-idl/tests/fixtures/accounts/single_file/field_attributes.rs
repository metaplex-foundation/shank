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
}

pub struct SomeWrapper<T>(pub T);