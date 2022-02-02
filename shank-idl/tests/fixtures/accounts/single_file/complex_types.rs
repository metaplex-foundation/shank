#[derive(ShankAccount)]
pub struct StructAccount {
    pub opt_vec_opt: Option<Vec<Option<u8>>>,
    pub vec_opt_pubkey: Vec<Option<Pubkey>>,
    pub opt_vec_custom_ty: Option<Vec<CustomType>>,
}
