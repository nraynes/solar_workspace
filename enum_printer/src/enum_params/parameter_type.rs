use syn::Ident;

#[derive(PartialEq, Eq, Hash)]
pub enum ParameterType {
    Attributes,
    Variants,
}

impl From<&Ident> for ParameterType {
    fn from(value: &Ident) -> Self {
        let name = value.to_string();
        match name.as_str() {
            "attributes" => Self::Attributes,
            "variants" => Self::Variants,
            _ => panic!("{} is not a valid option.", name),
        }
    }
}
