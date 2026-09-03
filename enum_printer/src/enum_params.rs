mod attributes;
mod parameter_type;
mod variants;

use std::collections::HashMap;

use attributes::Attributes;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    MetaNameValue, bracketed,
    parse::{Parse, ParseStream},
    parse2,
    token::Comma,
};

use crate::enum_params::{parameter_type::ParameterType, variants::Variants};

pub struct EnumParams {
    pub attributes: Option<Attributes>,
    pub variants: Option<Variants>,
}

impl Parse for EnumParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);
        let parameters = content
            .parse_terminated(MetaNameValue::parse, Comma)
            .expect(
                "Input to attribute print_enum must be a comma separated list of name-value pairs.",
            );
        let parameter_map: HashMap<ParameterType, TokenStream> =
            HashMap::from_iter(parameters.into_iter().map(|x| {
                (
                    ParameterType::from(x.path.get_ident().expect("Not a valid identity.")),
                    x.value.to_token_stream(),
                )
            }));

        if !input.is_empty() {
            return Err(input.error("The input is not empty at the end of the attribute."));
        }

        Ok(Self {
            attributes: parameter_map
                .get(&ParameterType::Attributes)
                .map(|t| parse2::<Attributes>(t.clone()).expect("Could not parse attributes.")),
            variants: parameter_map
                .get(&ParameterType::Variants)
                .map(|t| parse2::<Variants>(t.clone()).expect("Could not parse variants.")),
        })
    }
}
