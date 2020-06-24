extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn duktape_fn(_attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item.clone()).expect("failed to parse token stream as fn");
    let fn_name = ast.sig.ident;
    let fn_vis = ast.vis;
    let fn_name_str = syn::LitStr::new(&format!("{}", fn_name), fn_name.span());
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
                        std::ffi::CString::new(format!("{}", e)).into_raw(),
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
                        std::ffi::CString::new(format!("{}", e)).into_raw(),
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
        #fn_vis mod #fn_name {
            use super::duk;
            use super::#fn_name as fn_impl;

            pub struct DukFnImpl;

            unsafe impl duk::DukFunction for DukFnImpl {
                const NARGS: usize = #fn_arg_len;
                const NAME: &'static str = #fn_name_str;
                unsafe extern "C" fn duk_call(ctx: *mut duk_sys::duk_context) -> i32 {
                    #(
                        #args
                    )*
                    let res = fn_impl(#(
                        #arg_names,
                    )*);
                    #ret
                }
            }
        }
    };
    item.extend(TokenStream::from(gen));
    item
}
