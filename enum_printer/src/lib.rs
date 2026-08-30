mod attr_args;
mod enum_params;
mod print_enum_impl;

use attr_args::AttrArgs;
use proc_macro::TokenStream;
use syn::{ItemEnum, parse_macro_input};

#[proc_macro_attribute]
pub fn enum_printer(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemEnum);

    print_enum_impl::print_enum_impl(
        match attr.is_empty() {
            true => AttrArgs::empty(),
            false => parse_macro_input!(attr as AttrArgs),
        },
        item,
    )
    .into()
}
