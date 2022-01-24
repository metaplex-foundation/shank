/// NOTE: this was lifted from https://github.com/project-serum/anchor/blob/1a2fd38451b36a569287eb9794ec10e51675789e/lang/syn/src/idl/mod.rs
/// We want to add non-anchor specific extensions to this IDL which is why we don't depend on the
/// anchor crate instead.
use serde::{Deserialize, Serialize};

pub type IdlType = shank_macro_impl::types::RustType

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Idl {
    pub version: String,
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub constants: Vec<IdlConst>,
    pub instructions: Vec<IdlInstruction>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<IdlState>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub accounts: Vec<IdlTypeDefinition>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub types: Vec<IdlTypeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub events: Option<Vec<IdlEvent>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub errors: Option<Vec<IdlErrorCode>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlConst {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlState {
    #[serde(rename = "struct")]
    pub strct: IdlTypeDefinition,
    pub methods: Vec<IdlInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstruction {
    pub name: String,
    pub accounts: Vec<IdlAccountItem>,
    pub args: Vec<IdlField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccounts {
    pub name: String,
    pub accounts: Vec<IdlAccountItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdlAccountItem {
    IdlAccount(IdlAccount),
    IdlAccounts(IdlAccounts),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlEvent {
    pub name: String,
    pub fields: Vec<IdlEventField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlEventField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlType,
    pub index: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlTypeDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlTypeDefinitionTy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum IdlTypeDefinitionTy {
    Struct { fields: Vec<IdlField> },
    Enum { variants: Vec<IdlEnumVariant> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlEnumVariant {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fields: Option<EnumFields>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnumFields {
    Named(Vec<IdlField>),
    Tuple(Vec<IdlType>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlErrorCode {
    pub code: u32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub msg: Option<String>,
}
