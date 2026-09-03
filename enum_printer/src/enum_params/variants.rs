mod modification;

use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    token::Comma,
};

use modification::Modification;

pub struct Variants {
    pub modifications: Vec<Modification>,
}

impl Parse for Variants {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);

        let modifications = content
            .parse_terminated(Modification::parse, Comma)
            .expect("Variant modifications must be a comma separated list of meta-list values.");

        if !input.is_empty() {
            return Err(
                input.error("The input is not empty at the end of the variants configuration.")
            );
        }

        Ok(Self {
            modifications: modifications.into_iter().collect(),
        })
    }
}
