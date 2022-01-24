use syn::{Field, Ident};

// use crate::{attrs::TypeInfoMap, common::abort};
// use super::{dart_type::DartType, rust_type::RustType};

#[derive(Debug)]
pub struct ParsedStructField {
    pub ident: syn::Ident,
    // pub rust_type: RustType,
    // pub dart_type: DartType,
}

impl ParsedStructField {
    pub fn new(_f: &Field) -> Self {
        todo!()
        /*
        // unwrap is ok here since we only support structs with named fields
        let ident = f.ident.as_ref().unwrap().clone();
        let rust_type = RustType::from_type(&f.ty, type_infos);
        let rust_type = match rust_type {
            Some(x) => x,
            None => abort!(
                ident,
                "Not supporting custom types yet when deriving DartState"
            ),
        };
        let dart_type = DartType::from(&rust_type, type_infos);
        Self {
            ident,
            rust_type,
            dart_type,
        }
        */
    }

    pub fn method_ident(&self, owner_ident: &Ident) -> Ident {
        let method_prefix = format!("rid_{}", owner_ident.to_string().to_lowercase());
        let fn_name = format!("{}_{}", method_prefix, self.ident);
        Ident::new(&fn_name, self.ident.span())
    }
}
