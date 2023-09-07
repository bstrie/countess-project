use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ExprRange, Fields, ItemStruct, Lit, Type};
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn range(attr_tok: TokenStream, item_tok: TokenStream) -> TokenStream {
    //dbg!(&attr_tok);
    //dbg!(&item_tok);

    let attr_ast = parse_macro_input!(attr_tok as ExprRange);
    let item_ast = parse_macro_input!(item_tok as ItemStruct);

    //dbg!(&attr_ast);
    //dbg!(&item_ast);

    let ident = item_ast.ident;

    let num_type = {
        let Fields::Unnamed(fields) = item_ast.fields else { panic!() };
        let field = fields.unnamed.first().unwrap().clone();
        field.ty
    };

    //dbg!(ident);
    //dbg!(num_type);

    //let min = i8::MAX.to_token_stream();
    let min = match attr_ast.start {
        Some(n) => n, // TODO: validate number is appropriate
        _ => todo!()
    };
    let max = attr_ast.end;
    //dbg!(min);
    //dbg!(max);

    quote! {
        struct #ident {
            v: #num_type,
        }

        impl #ident {
            const MIN: #num_type = #min;
            const MAX: #num_type = #max;
        }
    }.into()
}
