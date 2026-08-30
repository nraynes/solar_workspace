use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Variant, punctuated::Punctuated, token::Comma};

use crate::AttrArgs;

pub fn print_enum_impl(attr: AttrArgs, mut item: syn::ItemEnum) -> TokenStream {
    let mut enum_map_with_variants: HashMap<Ident, Punctuated<Variant, Comma>> = HashMap::new();

    // Add all of the enums and variants to hashmap.
    for variant in &mut item.variants {
        if let Some(print_to_enums_attr) = variant
            .attrs
            .iter()
            .find(|&x| x.path().is_ident("print_to_enum"))
        {
            let enums_for_this_variant = print_to_enums_attr
                .parse_args_with(Punctuated::<Ident, Comma>::parse_terminated)
                .expect("Input to attribute print_to_enum must be a comma separated list.");

            // Remove print_to_enum attribute only.
            variant
                .attrs
                .retain(|x| !x.path().is_ident("print_to_enum"));

            for enum_parent_ident in enums_for_this_variant.iter() {
                if !enum_map_with_variants.contains_key(enum_parent_ident) {
                    enum_map_with_variants.insert(
                        enum_parent_ident.clone(),
                        Punctuated::<Variant, Comma>::new(),
                    );
                }
                enum_map_with_variants
                    .get_mut(enum_parent_ident)
                    .expect("Could not process enum list from variant attribute.")
                    .push(variant.clone());
            }
        }
    }

    // Quote all of the enum copies.
    let mut enum_copies = Vec::new();
    for (enum_ident, list_of_variants) in enum_map_with_variants {
        let mut item_enum = item.clone();
        item_enum.variants = list_of_variants;
        if let Some(parameter_map) = &attr.enum_data
            && let Some(enum_parameters) = parameter_map.get(&enum_ident)
        {
            if let Some(attributes) = &enum_parameters.attributes {
                item_enum.attrs = attributes.list.clone();
            }
            if let Some(variants) = &enum_parameters.variants {
                for modification in &variants.modifications {
                    modification.modify(&mut item_enum);
                }
            }
        }
        item_enum.ident = enum_ident;
        enum_copies.push(quote! { #item_enum });
    }

    quote! {
        #(#enum_copies)*
    }
}
