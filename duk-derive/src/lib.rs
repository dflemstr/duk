extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn duktape_fn(_attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item.clone()).expect("failed to parse token stream as fn");
    let fn_name = ast.sig.ident;
    let duk_fn_name = syn::Ident::new(
        &format!("duk_derive_{}", fn_name),
        proc_macro2::Span::call_site(),
    );
    let fn_arg_len_name = syn::Ident::new(
        &format!("{}_arg_len", duk_fn_name),
        proc_macro2::Span::call_site(),
    );
    let mut arg_names = Vec::new();
    let mut args = Vec::new();
    for arg in ast.sig.inputs {
        let arg = match arg {
            syn::FnArg::Receiver(_) => {
                panic!("cannot derive `duktape_fn` on function that takes `self` as an argument")
            }
            syn::FnArg::Typed(a) => a,
        };
        let name = format!("arg_{}", args.len());
        let name_ident = syn::Ident::new(&name, arg.span());
        let arg_idx = args.len() as i32;
        arg_names.push(name_ident.clone());
        args.push(quote! {
            let #name_ident = match duk::deserialize_from_stack(ctx, #arg_idx) {
                Ok(a) => a,
                Err(e) => {
                    duk::duk_sys::duk_push_error_object(
                        ctx,
                        duk::duk_sys::DUK_ERR_TYPE_ERROR as i32,
                        format!("{}", e).as_ptr().cast()
                    );
                    duk::duk_sys::duk_throw_raw(ctx);
                    return 0;
                }
            };
        });
    }
    let fn_arg_len = args.len();
    let ret = match ast.sig.output {
        syn::ReturnType::Type(_, _) => quote! {
            match duk::serialize_to_stack(ctx, &res) {
                Ok(_) => {
                    1
                },
                Err(e) => {
                    duk::duk_sys::duk_push_error_object(
                        ctx,
                        duk::duk_sys::DUK_ERR_TYPE_ERROR as i32,
                        format!("{}", e).as_ptr().cast()
                    );
                    duk::duk_sys::duk_throw_raw(ctx);
                    0
                }
            }
        },
        _ => quote! {
            0
        },
    };
    let gen = quote! {
        unsafe extern "C" fn #duk_fn_name(ctx: *mut duk_sys::duk_context) -> i32 {
            #(
                #args
            )*

            let res = #fn_name(#(
                #arg_names,
            )*);

            #ret
        }
        const #fn_arg_len_name: usize = #fn_arg_len;
    };
    item.extend(TokenStream::from(gen));
    item
}

struct AddGlobalFn {
    ctx: syn::Expr,
    func: syn::Path,
}
impl Parse for AddGlobalFn {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let ctx = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let func = input.parse()?;
        Ok(AddGlobalFn { ctx, func })
    }
}

#[proc_macro]
pub fn add_global_fn(input: TokenStream) -> TokenStream {
    let AddGlobalFn { ctx, func } = syn::parse_macro_input!(input as AddGlobalFn);

    let leaf = func.segments.iter().cloned().next_back().unwrap().ident;

    let derived = syn::Path {
        leading_colon: func.leading_colon,
        segments: func
            .segments
            .iter()
            .cloned()
            .take(func.segments.len() - 1)
            .chain(std::iter::once(syn::PathSegment {
                ident: syn::Ident::new(&format!("duk_derive_{}", leaf), leaf.span()),
                arguments: syn::PathArguments::None,
            }))
            .collect(),
    };
    let derived_arg_len = syn::Path {
        leading_colon: func.leading_colon,
        segments: func
            .segments
            .iter()
            .cloned()
            .take(func.segments.len() - 1)
            .chain(std::iter::once(syn::PathSegment {
                ident: syn::Ident::new(&format!("duk_derive_{}_arg_len", leaf), leaf.span()),
                arguments: syn::PathArguments::None,
            }))
            .collect(),
    };

    (quote! {
        unsafe {
            #ctx.add_global_fn(stringify!(#leaf), #derived, #derived_arg_len)
        }
    })
    .into()
}
