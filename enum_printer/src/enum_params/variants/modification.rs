mod append_tuple;

use append_tuple::AppendTuple;
use quote::ToTokens;
use syn::{Expr, Ident, ItemEnum, parenthesized, parse::Parse, parse2};

#[derive(PartialEq, Eq, Hash)]
pub enum Modification {
    AppendTuple(AppendTuple),
}

impl Modification {
    pub fn modify(&self, item_enum: &mut ItemEnum) {
        match self {
            Self::AppendTuple(x) => x.modify(item_enum),
        };
    }
}

impl Parse for Modification {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        let inner: Expr = content.parse()?;

        if !input.is_empty() {
            return Err(input.error("The input is not empty at the end of the modification"));
        }

        match name.to_string().as_str() {
            "append_tuple" => Ok(Self::AppendTuple(parse2::<AppendTuple>(
                inner.to_token_stream(),
            )?)),
            _ => Err(syn::Error::new(
                name.span(),
                format!("{} is not a valid option.", name),
            )),
        }
    }
}
