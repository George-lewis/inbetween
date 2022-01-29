use proc_macro::TokenStream;
use quote::quote;
use syn::{BinOp, Expr};

macro_rules! error {
    ($msg:expr) => {
        quote::quote! { compile_error!($msg) }.into()
    };
}

#[derive(Debug)]
enum Wrap<'a> {
    Exprs(&'a Expr),
    Sig(BinOp),
}

impl Wrap<'_> {
    fn expr(&self) -> Result<&Expr, TokenStream> {
        match self {
            Wrap::Exprs(e) => Ok(e),
            Wrap::Sig(_) => Err(error!("Expected expr")),
        }
    }
    fn sig(&self) -> Result<&BinOp, TokenStream> {
        match self {
            Wrap::Exprs(_) => Err(error!("Expected op")),
            Wrap::Sig(s) => Ok(s),
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
        Expr::Lit(_) | Expr::Path(_) | Expr::Field(_) | Expr::Call(_) => {
            vec![Wrap::Exprs(expr)]
        }
        Expr::MethodCall(emc) => extract(emc.receiver.as_ref()),
        _ => vec![],
    }
}

macro_rules! expr {
    ($e:expr) => {
        match $e.expr() {
            Ok(x) => x,
            Err(e) => return e,
        }
    };
}

macro_rules! op {
    ($e:expr) => {
        match $e.sig() {
            Ok(x) => x,
            Err(e) => return e,
        }
    };
}

#[proc_macro]
pub fn between(input: TokenStream) -> TokenStream {
    let ast: Expr = syn::parse(input).unwrap();

    let sig = extract(&ast);

    if sig.len() < 5 {
        return error!("Not enough tokens");
    }

    if sig.len() > 5 {
        return error!("Too many tokens");
    }

    let min = expr!(sig[0]);
    let op1 = op!(sig[1]);
    let var = expr!(sig[2]);
    let op2 = op!(sig[3]);
    let max = expr!(sig[4]);

    let quote = quote! {
        #min #op1 #var && #var #op2 #max
    };
    quote.into()
}
