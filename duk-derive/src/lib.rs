extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn duktape_fn(_attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let duk_path = if std::env::var("CARGO_PKG_NAME").unwrap() == "duk" {
        quote! { crate }
    } else {
        quote! { ::duk }
    };
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
            let #name_ident = match #duk_path::deserialize_from_stack(ctx, #arg_idx) {
                Ok(a) => a,
                Err(e) => {
                    #duk_path::duk_sys::duk_push_error_object(
                        ctx,
                        #duk_path::duk_sys::DUK_ERR_TYPE_ERROR as i32,
                        std::ffi::CString::new(format!("{}", e)).unwrap_or_default().into_raw(),
                    );
                    #duk_path::duk_sys::duk_throw_raw(ctx);
                    return 0;
                }
            };
        });
    }
    let fn_arg_len = args.len();
    let ret = match ast.sig.output {
        syn::ReturnType::Type(_, _) => quote! {
            match #duk_path::serialize_to_stack(ctx, &res) {
                Ok(_) => {
                    1
                },
                Err(e) => {
                    #duk_path::duk_sys::duk_push_error_object(
                        ctx,
                        #duk_path::duk_sys::DUK_ERR_TYPE_ERROR as i32,
                        std::ffi::CString::new(format!("{}", e)).unwrap_or_default().into_raw(),
                    );
                    #duk_path::duk_sys::duk_throw_raw(ctx);
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
            use super::#fn_name as fn_impl;

            pub struct DukFnImpl;

            unsafe impl #duk_path::DukFunction for DukFnImpl {
                const NARGS: usize = #fn_arg_len;
                const NAME: &'static str = #fn_name_str;
                unsafe extern "C" fn duk_call(ctx: *mut #duk_path::duk_sys::duk_context) -> i32 {
                    #(
                        #args
                    )*
                    let res = match std::panic::catch_unwind(|| fn_impl(#(
                        #arg_names,
                    )*)) {
                        Ok(res) => res,
                        Err(e) => {
                            let error = if let Some(msg) = e.downcast_ref::<&str>() {
                                format!("panic: {}", *msg)
                            } else if let Some(msg) = e.downcast_ref::<String>() {
                                format!("panic: {}", msg)
                            } else {
                                "panic: unknown error".into()
                            };
                            #duk_path::duk_sys::duk_push_error_object(
                                ctx,
                                #duk_path::duk_sys::DUK_ERR_ERROR as i32,
                                std::ffi::CString::new(error).unwrap_or_default().into_raw(),
                            );
                            #duk_path::duk_sys::duk_throw_raw(ctx);
                            return 0;
                        }
                    };
                    #ret
                }
            }
        }
    };
    item.extend(TokenStream::from(gen));
    item
}
