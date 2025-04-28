use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use syn::{Lifetime, TypeReference};

#[derive(Clone, PartialEq, Eq)]
pub enum ParsedReference {
    Owned,
    Ref(Option<syn::Ident>),
    RefMut(Option<syn::Ident>),
}

impl Hash for ParsedReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use discriminant to hash the enum variant
        std::mem::discriminant(self).hash(state);

        // Hash the inner lifetime if present
        match self {
            ParsedReference::Owned => {}
            ParsedReference::Ref(lifetime) => {
                if let Some(lt) = lifetime {
                    lt.to_string().hash(state);
                }
            }
            ParsedReference::RefMut(lifetime) => {
                if let Some(lt) = lifetime {
                    lt.to_string().hash(state);
                }
            }
        }
    }
}

impl Debug for ParsedReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            ParsedReference::Owned => "ParsedReference::Owned".to_string(),
            ParsedReference::Ref(ident) => {
                format!("ParsedReference::Ref({:?})", ident)
            }
            ParsedReference::RefMut(ident) => {
                format!("ParsedReference::RefMut({:?})", ident)
            }
        };
        write!(f, "{}", r)
    }
}

impl From<&TypeReference> for ParsedReference {
    fn from(r: &TypeReference) -> Self {
        let TypeReference {
            lifetime,
            mutability,
            ..
        } = r;

        let lifetime_ident = lifetime
            .as_ref()
            .map(|Lifetime { ident, .. }| ident.clone());

        match mutability.is_some() {
            true => ParsedReference::RefMut(lifetime_ident),
            false => ParsedReference::Ref(lifetime_ident),
        }
    }
}

impl ParsedReference {
    pub fn with_lifetime(self, lifetime: syn::Ident) -> Self {
        match self {
            ParsedReference::Owned => self,
            ParsedReference::Ref(_) => ParsedReference::Ref(Some(lifetime)),
            ParsedReference::RefMut(_) => {
                ParsedReference::RefMut(Some(lifetime))
            }
        }
    }

    /**
     * Adds the provided lifetime if this is a Ref or RefMut and has no lifteime already.
     * Otherwise returns itself unchanged.
     */
    pub fn ensured_lifetime(&self, lifetime: syn::Ident) -> Self {
        match self {
            ParsedReference::Ref(None) => ParsedReference::Ref(Some(lifetime)),
            ParsedReference::RefMut(None) => {
                ParsedReference::RefMut(Some(lifetime))
            }
            _ => self.clone(),
        }
    }

    pub fn lifetime(&self) -> Option<&syn::Ident> {
        match self {
            ParsedReference::Owned => None,
            ParsedReference::Ref(lifetime) => lifetime.as_ref(),
            ParsedReference::RefMut(lifetime) => lifetime.as_ref(),
        }
    }
}
