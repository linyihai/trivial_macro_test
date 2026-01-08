use proc_macro::TokenStream;
use quote::{format_ident, quote};

use syn::{FnArg, ItemFn, Pat, PatType, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn entry_call_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let fn_name = &sig.ident;
    let fn_output = &sig.output;
    let (impl_generics, _ty_generics, _where_clause) = sig.generics.split_for_impl();

    let (param_names, param_types): (Vec<_>, Vec<_>) = sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => match &**pat {
                Pat::Ident(ident) => Some((ident.ident.clone(), ty.clone())),
                _ => None,
            },
            FnArg::Receiver(_) => None,
        })
        .unzip();

    let entry_name = format_ident!("entry_{}", fn_name);

    let expanded = {
        let ty_arrs = literal_types(&param_types);
        if ty_arrs.len() != param_names.len() {
            panic!("Unsupported parameter type detected!");
        }

        let params = call_params(&ty_arrs);
        quote! {
            pub fn #entry_name #impl_generics (args: &mut Args, ctx: &mut TxContext) #fn_output {
                let mut instance = bcs::from_bytes::<Self>(&args.next().expect("Failed to parse BCS")).unwrap();
                Self::#fn_name(&mut instance, #(#params),*)
            }

            #input
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn entry_call_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let fn_name = &sig.ident;
    let fn_output = &sig.output;
    let (impl_generics, _ty_generics, _where_clause) = sig.generics.split_for_impl();

    let (param_names, param_types): (Vec<_>, Vec<_>) = sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => match &**pat {
                Pat::Ident(ident) => Some((ident.ident.clone(), ty.clone())),
                _ => None,
            },
            FnArg::Receiver(_) => None,
        })
        .unzip();

    let entry_name = format_ident!("entry_{}", fn_name);
    let expanded = {
        let ty_arrs = literal_types(&param_types);
        if ty_arrs.len() != param_names.len() {
            panic!("Unsupported parameter type detected!");
        }

        let params = call_params(&ty_arrs);
        quote! {
            pub fn #entry_name #impl_generics (args: &mut Args, ctx: &mut TxContext) #fn_output {
                #fn_name(#(#params),*)
            }

            #input
        }
    };

    TokenStream::from(expanded)
}

struct TypeInfo {
    name: String,
    token: proc_macro2::TokenStream,
    mutable_ref: bool,
    ref_inst: bool,
}

fn literal_types(tys: &[Box<Type>]) -> Vec<TypeInfo> {
    let mut ty_arrs = vec![];
    for ty in tys {
        match ty.as_ref() {
            syn::Type::Path(type_path) => {
                println!("{:?}", type_path);
                if let Some(segment) = type_path.path.segments.last() {
                    ty_arrs.push(TypeInfo {
                        name: segment.ident.to_string(),
                        token: quote! { #type_path },
                        mutable_ref: false,
                        ref_inst: false,
                    });
                }
            }
            syn::Type::Reference(type_ref) => {
                println!("{:?}", type_ref);
                if let syn::Type::Path(type_path) = &*type_ref.elem {
                    if let Some(segment) = type_path.path.segments.last() {
                        ty_arrs.push(TypeInfo {
                            name: segment.ident.to_string(),
                            token: quote! { #type_path },
                            mutable_ref: type_ref.mutability.is_some(),
                            ref_inst: true,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    ty_arrs
}

fn call_params(ty_arrs: &[TypeInfo]) -> Vec<proc_macro2::TokenStream> {
    let mut params = vec![];
    for ty in ty_arrs {
        let ty_token = &ty.token;
        let s = match ty.name.as_str() {
            "u64" => quote! {
                fast_ascii_to_u64(&args.next_pure().expect("Failed to parse u64"))
            },
            "u32" => quote! {
                fast_ascii_to_u64(&args.next_pure().expect("Failed to parse u32")) as u32
            },
            "bool" => quote! {
                (fast_ascii_to_u64(&args.next_pure().expect("Failed to parse bool"))) != 0
            },
            "TxContext" => quote! {
                ctx
            },
            _ => match (ty.ref_inst, ty.mutable_ref) {
                (true, true) => quote! {
                    &mut bcs::from_bytes::<#ty_token>(&args.next().expect("Failed to parse BCS")).unwrap()
                },
                (true, false) => quote! {
                    &bcs::from_bytes::<#ty_token>(&args.next().expect("Failed to parse BCS")).unwrap()
                },
                (false, true) => {
                    quote! {
                        mut bcs::from_bytes::<#ty_token>(&args.next().expect("Failed to parse BCS")).unwrap()
                    }
                }
                (false, false) => {
                    quote! {
                        bcs::from_bytes::<#ty_token>(&args.next().expect("Failed to parse BCS")).unwrap()
                    }
                }
            },
        };
        params.push(s);
    }
    params
}
