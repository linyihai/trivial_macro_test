use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, PatType, parse_macro_input};

#[proc_macro_attribute]
pub fn entry_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. 将输入的代码解析为函数结构
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis; // 函数可见性
    let sig = &input.sig; // 函数签名
    let body = &input.block; // 函数体
    let fn_name = &sig.ident; // 函数名
    let fn_attrs = &input.attrs;
    let param_inputs = &sig.inputs;
    let fn_output = &sig.output;

    // 提取参数信息
    let (param_names, param_types): (Vec<_>, Vec<_>) = sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => match &**pat {
                Pat::Ident(ident) => Some((ident.ident.clone(), ty.clone())),
                _ => None,
            },
            FnArg::Receiver(_) => None, // 忽略 self 参数
        })
        .unzip();

    // 2. 重新构建函数，注入装饰逻辑
    let entry_name = format_ident!("entry_{}", fn_name);
    let mut call_params = param_inputs.clone(); // 直接使用参数输入 ;
    call_params.pop();
    let expanded = {
        let mut ty_arrs = vec![];
        for ty in param_types {
            // println!("Parameter: {} of type {:?}", name, ty);
            match ty.as_ref() {
                syn::Type::Path(type_path) => {
                    if let Some(segment) = type_path.path.segments.last() {
                        ty_arrs.push(segment.ident.to_string());
                        println!("Type segment: {}", segment.ident);
                    }
                }
                _ => {}
            }
        }
        if ty_arrs.len() != param_names.len() {
            panic!("Unsupported parameter type detected!");
        }
        let mut params = vec![];
        for (_, ty) in param_names.iter().zip(ty_arrs.iter()) {
            let ty_name = format_ident!("{}", ty);
            let s = match ty.as_str() {
                "u64" => quote! {
                    u64::from_ascii(&args.next_pure().expect("Failed to parse u64")).unwrap()
                },
                "u32" => quote! {
                    u64::from_ascii(&args.next_pure().expect("Failed to parse u32")).unwrap() as u32
                },
                "bool" => quote! {
                    (u64::from_ascii(&args.next_pure().expect("Failed to parse bool")).unwrap()) != 0
                },
                _ => quote! {
                    bcs::from_bytes::<#ty_name>(&args.next().expect("Failed to parse BCS")).unwrap()
                },
            };
            params.push(s);
        }

        quote! {
            pub fn #entry_name(&mut self, args: &mut Args) {
                let result = self.#fn_name(#(#params),*);
            }

            #(#fn_attrs)*
            #vis fn #fn_name(#param_inputs) #fn_output {
                #body
            }
        }
    };

    // 3. 返回生成的 TokenStream
    TokenStream::from(expanded)
}
