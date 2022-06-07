use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Pat, PatTupleStruct, Token};

extern crate proc_macro2;

struct Unpattern {
    idents: Vec<Ident>,
    pat: Pat,
    expr: Expr,
}

fn traverse_pat(pat: &Pat, idents: &mut Vec<Ident>) {
    match pat {
        Pat::Box(_) => println!("box"),
        Pat::Ident(ident) => {
            println!("ident: {}", ident.ident);
            idents.push(ident.ident.clone());
        }
        Pat::Lit(_) => println!("lit"),
        Pat::Macro(_) => println!("macro"),
        Pat::Or(_) => println!("or"),
        Pat::Path(_) => println!("path"),
        Pat::Range(_) => println!("range"),
        Pat::Reference(_) => println!("reference"),
        Pat::Rest(_) => println!("rest"),
        Pat::Slice(_) => println!("slice"),
        Pat::Struct(_) => println!("struct"),
        Pat::Tuple(_) => println!("tuple"),
        Pat::TupleStruct(PatTupleStruct { path, pat, .. }) => {
            let seg = path.segments.to_token_stream();
            pat.elems.iter().for_each(|p| traverse_pat(p, idents));
            println!("tuple struct: {}", seg);
        }
        Pat::Type(_) => println!("type"),
        Pat::Verbatim(_) => println!("verbatim"),
        Pat::Wild(_) => println!("wild"),
        _ => println!("other"),
    }
}

impl Parse for Unpattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.lookahead1();
        let pat = input.parse::<Pat>()?;
        input.parse::<Token![<-]>()?;
        let expr = input.parse::<Expr>()?;
        let mut idents = vec![];
        traverse_pat(&pat, &mut idents);
        Ok(Unpattern { idents, pat, expr })
    }
}

impl ToTokens for Unpattern {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let idents = &self.idents;
        let pat = &self.pat;
        let expr = &self.expr;

        tokens.extend(quote! {
            #[allow(irrefutable_let_patterns)]
            let #(#idents),* = if let #pat = #expr {
                #(#idents),*
            } else {
                unreachable!("The pattern isn't match with the expression");
            };
        });
    }
}

#[proc_macro]
pub fn unpat(token: TokenStream) -> TokenStream {
    let v = parse_macro_input!(token as Unpattern);
    v.to_token_stream().into()
}
