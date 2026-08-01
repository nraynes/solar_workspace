use proc_macro2::Span;
use syn::{
    Fields, Ident, ItemEnum, Type,
    parse::{Parse, ParseStream},
};

#[derive(PartialEq, Eq, Hash)]
pub struct AppendTuple {
    pub what: Ident,
}

impl AppendTuple {
    pub fn modify(&self, item_enum: &mut ItemEnum) {
        for variant in &mut item_enum.variants {
            if let Fields::Unnamed(fields) = &mut variant.fields {
                for unnamed_field in &mut fields.unnamed {
                    if let Type::Path(ty) = &mut unnamed_field.ty {
                        if let Some(segment) = ty.path.segments.last_mut() {
                            segment.ident = Ident::new(
                                format!("{}{}", segment.ident, self.what).as_str(),
                                Span::call_site(),
                            )
                        }
                    }
                }
            }
        }
    }
}

impl Parse for AppendTuple {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let what: Ident = input
            .parse()
            .expect("Could not parse identity from append directive.");

        if !input.is_empty() {
            return Err(input.error("The input is not empty at the end of the attribute"));
        }

        Ok(Self { what })
    }
}
