use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{braced, Attribute, Generics, Ident, Token, Visibility, WhereClause};

pub struct PhantomType {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub kind: Kind,
    pub ident: Ident,
    pub generics: Generics,
}

pub enum Kind {
    UnitStruct(Token![struct]),
    VoidEnum(Token![enum]),
}

impl Parse for PhantomType {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;

        let lookahead = input.lookahead1();
        let kind = if lookahead.peek(Token![struct]) {
            input.parse().map(Kind::UnitStruct)?
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(Kind::VoidEnum)?
        } else {
            return Err(lookahead.error());
        };

        let ident: Ident = input.parse()?;

        // Require there to be generics.
        input.fork().parse::<Token![<]>()?;
        let generics: Generics = input.parse()?;
        let where_clause: Option<WhereClause> = input.parse()?;

        match kind {
            Kind::UnitStruct(_) => {
                input.parse::<Token![;]>()?;
            }
            Kind::VoidEnum(_) => {
                let content;
                let brace_token = braced!(content in input);
                if !content.is_empty() {
                    return Err(Error::new(
                        brace_token.span.join(),
                        "phantom enum must have 0 variants",
                    ));
                }
            }
        }

        Ok(PhantomType {
            attrs,
            vis,
            kind,
            ident,
            generics: Generics {
                where_clause,
                ..generics
            },
        })
    }
}
