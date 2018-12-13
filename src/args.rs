use syn::parse::{Parse, ParseStream, Result};

pub struct Nothing;

impl Parse for Nothing {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(Nothing)
    }
}
