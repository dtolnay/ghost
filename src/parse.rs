use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Generics, Ident, Token, Visibility, WhereClause};

pub struct UnitStruct {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        input.parse::<Token![struct]>()?;
        let ident: Ident = input.parse()?;

        // Require there to be generics.
        input.fork().parse::<Token![<]>()?;
        let generics: Generics = input.parse()?;
        let where_clause: Option<WhereClause> = input.parse()?;

        input.parse::<Token![;]>()?;

        Ok(UnitStruct {
            attrs: attrs,
            vis: vis,
            ident: ident,
            generics: Generics {
                where_clause: where_clause,
                ..generics
            },
        })
    }
}
