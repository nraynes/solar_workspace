use std::collections::HashMap;

use quote::ToTokens;
use syn::{
    Ident, MetaNameValue,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Comma,
};

use crate::enum_params::EnumParams;

pub struct AttrArgs {
    pub enum_data: Option<HashMap<Ident, EnumParams>>,
}

impl AttrArgs {
    pub fn empty() -> Self {
        Self { enum_data: None }
    }
}

impl Parse for AttrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parameter_list = Punctuated::<MetaNameValue, Comma>::parse_terminated(input)
            .expect("Input to enum_printer must be a comma separated list.");
        let enum_data: Option<HashMap<Ident, EnumParams>> =
            Some(HashMap::from_iter(parameter_list.into_iter().map(|x| {
                (
                    x.path.get_ident().expect("Not a valid identity.").clone(),
                    parse2::<EnumParams>(x.value.to_token_stream())
                        .expect("Could not parse parameters to attribute enum_printer."),
                )
            })));
        Ok(Self { enum_data })
    }
}
