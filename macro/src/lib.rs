use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{parse::{ParseStream, Parse}, punctuated::Punctuated, Result, Expr, Token, Member, parse_macro_input};

struct CloneItem {
    expr: Box<dyn ToTokens>,
    name: Box<dyn ToTokens>,
}

struct CloneItems {
    to_clone: Vec<CloneItem>,
}

impl Parse for CloneItems {
    fn parse(input: ParseStream) -> Result<Self> {
        let input =
            Punctuated::<Expr, Token![,]>::parse_terminated(input)
                .expect("Arguments should form a comma (',') separated list.");
        let buffer = input
            .into_iter()
            .enumerate()
            .map(|(debug_index, expr)| {
                match expr {
                    Expr::Assign(a) => {
                        let name = a.left;
                        let expr: Box<dyn ToTokens> = match *a.right {
                            Expr::Field(e) => Box::new(e),
                            Expr::Path(p) => Box::new(p),
                            _ => panic!("Couldn't parse righthand side of an assignment on position {debug_index}. Is neither field nor local variable."),
                        };
                        CloneItem { name, expr }
                    },
                    Expr::Field(e) => {
                        let name = Box::new(match e.member.clone() {
                            Member::Named(x) => x,
                            Member::Unnamed(_) => panic!("Tuple fields are unsupported. Use a direct assignment '=' instead."),
                        });
                        let expr = Box::new(e);
                        CloneItem { name, expr }
                    },
                    Expr::Path(p) => CloneItem { expr: Box::new(p.clone()), name: Box::new(p), },
                    _ => panic!("Couldn't parse item on position {debug_index}. It's neither field nor local variable."),
                }
            })
            .collect();
        Ok(Self { to_clone: buffer })
    }
}

/// Reduces boilerplate when cloning objects.
///
/// Turns
///```ignore
/// clone!(self.broker, local, local.indirection.nested, named = tuple.1);
///```
/// into
///```ignore
/// let broker = self.broker.clone();
/// let local = local.clone();
/// let nested = local.indirection.nested.clone();
/// let named = tuple.1.clone();
///```
/// You can assign custom names using ```<name> = <expression>``` e.g. ```cloned!(my_broker = self.broker);```
///
/// The macro expects a comma separated list of field accesses or locals. Tuple field accesses must be explicitly named using '='.
/// ```clone!(tuple.1);``` won't work, but ```clone!(broker = tuple.1);``` will.
#[proc_macro]
pub fn clone(item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as CloneItems);
    parsed
        .to_clone
        .into_iter()
        .map(|item| {
            let name = item.name;
            let expr = item.expr;
            quote! { let #name = #expr.clone(); }
        })
        .fold(quote! {}, |sum, element| {
            quote! {
                #sum
                #element
            }
        })
        .into()
}
