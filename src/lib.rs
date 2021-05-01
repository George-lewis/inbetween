

use proc_macro::TokenStream;
use syn::{BinOp, Expr};
use quote::quote;

#[derive(Debug)]
enum Wrap<'a> {
    Exprs(&'a Expr),
    Sig(BinOp)
}

impl Wrap<'_> {
    fn expr(&self) -> &Expr {
        match self {
            Wrap::Exprs(e) => e,
            Wrap::Sig(_) => panic!()
        }
    }
    fn sig(&self) -> &BinOp {
        match self {
            Wrap::Exprs(_) => panic!(),
            Wrap::Sig(s) => s
        }
    }
}

fn extract(expr: &Expr) -> Vec<Wrap> {
    match expr {
        Expr::Binary(eb) => {
            let mut v = vec![];
            v.extend(extract(eb.left.as_ref()));
            v.push(Wrap::Sig(eb.op));
            v.extend(extract(eb.right.as_ref()));
            v
        }
        Expr::Lit(_) | Expr::Path(_) => {
            vec![Wrap::Exprs(expr)]
        }
        Expr::MethodCall(emc) => {
            extract(emc.receiver.as_ref())
        }
        _ => vec![]
    }
}

#[proc_macro]
pub fn between(input: TokenStream) -> TokenStream {
    let ast: Expr = syn::parse(input).unwrap();

    let sig = extract(&ast);

    let min = sig[0].expr();
    let op1 = sig[1].sig();
    let var = sig[2].expr();
    let op2 = sig[3].sig();
    let max = sig[4].expr();

    let quote = quote! {
        #min #op1 #var && #var #op2 #max
    };
    quote.into()
}