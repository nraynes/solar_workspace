use syn::{
    Attribute, Meta, bracketed,
    parse::{Parse, ParseStream},
    token::{Bracket, Comma, Pound},
};

pub struct Attributes {
    pub list: Vec<Attribute>,
}

fn convert_meta_to_attribute(meta: &Meta) -> Attribute {
    Attribute {
        pound_token: Pound::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Bracket::default(),
        meta: meta.clone(),
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);

        let attribute_meta = content
            .parse_terminated(Meta::parse, Comma)
            .expect("Could not parse list of attributes.");
        let list: Vec<Attribute> = attribute_meta
            .iter()
            .map(convert_meta_to_attribute)
            .collect();

        if !input.is_empty() {
            return Err(input.error("The input is not empty at the end of the attribute"));
        }

        Ok(Self { list })
    }
}
