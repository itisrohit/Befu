use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ExprPath, FnArg, ItemFn, Meta, Pat, ReturnType, Token};

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let original_fn = input.clone();
    let name = &input.sig.ident;
    let vis = &input.vis;
    let wrapper_name = format_ident!("__befu_wrapper_{}", name);
    let metadata_name = format_ident!("__befu_metadata_{}", name);

    // Parse attribute for custom name: #[command(name = "custom.command")]
    let mut custom_name = None;
    if !attr.is_empty() {
        let attr_parser = syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated;
        let attrs = attr_parser.parse(attr).expect("Failed to parse command attributes");
        for meta in attrs {
            if let Meta::NameValue(nv) = meta {
                if nv.path.is_ident("name") {
                    if let Expr::Lit(expr_lit) = nv.value {
                        if let syn::Lit::Str(lit) = expr_lit.lit {
                            custom_name = Some(lit.value());
                        }
                    }
                }
            }
        }
    }

    let command_name_str = custom_name.unwrap_or_else(|| name.to_string());

    // Extract doc comments for metadata description
    let mut description = String::new();
    for attr in &input.attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        let doc = lit_str.value().trim().to_string();
                        if !doc.is_empty() && description.is_empty() {
                            description = doc;
                        }
                    }
                }
            }
        }
    }
    if description.is_empty() {
        description = format!("The {} command", name);
    }

    let description_lit = syn::LitStr::new(&description, name.span());

    // Extract arguments for the Arguments struct
    let mut args_fields = Vec::new();
    let mut args_names = Vec::new();

    for arg in &input.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_id) = &*pat_type.pat {
                let arg_name = &pat_id.ident;
                let arg_type = &pat_type.ty;
                args_fields.push(quote! { pub #arg_name: #arg_type });
                args_names.push(arg_name);
            }
        }
    }

    let has_args = !args_names.is_empty();
    let args_struct_name = format_ident!("__BefuArgs_{}", name);

    let args_parsing = if has_args {
        quote! {
            #[derive(serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            struct #args_struct_name {
                #(#args_fields),*
            }
            let args_val = req.args.clone().unwrap_or(serde_json::Value::Null);
            let args: #args_struct_name = match serde_json::from_value(args_val.clone()) {
                Ok(a) => a,
                Err(e) => return befu_bridge::failure_response(&req.id, "INVALID_ARGUMENT", e.to_string(), Some(args_val)),
            };
        }
    } else {
        quote! {
            let args_val = req.args.clone().unwrap_or(serde_json::Value::Null);
            if !args_val.is_null() && !args_val.as_object().map_or(false, |o| o.is_empty()) {
                return befu_bridge::failure_response(
                    &req.id,
                    "INVALID_ARGUMENT",
                    "This command does not accept arguments".to_owned(),
                    Some(args_val),
                );
            }
        }
    };

    let call_args = args_names.iter().map(|name| quote! { args.#name });

    let return_response = match &input.sig.output {
        ReturnType::Default => {
            quote! { befu_bridge::success_response(&req.id, serde_json::Value::Null) }
        }
        ReturnType::Type(_, _) => quote! {
            match serde_json::to_value(result) {
                Ok(val) => befu_bridge::success_response(&req.id, val),
                Err(e) => befu_bridge::failure_response(&req.id, "SERIALIZATION_ERROR", e.to_string(), None),
            }
        },
    };

    let result_assignment = if let ReturnType::Default = &input.sig.output {
        quote! { #name(#(#call_args),*); }
    } else {
        quote! { let result = #name(#(#call_args),*); }
    };

    let expanded = quote! {
        #original_fn

        #[allow(non_camel_case_types)]
        #vis fn #wrapper_name(req: &befu_bridge::BridgeRequest) -> befu_bridge::BridgeResponse {
            #[allow(non_camel_case_types)]
            #args_parsing
            #result_assignment
            #return_response
        }

        #[allow(non_camel_case_types)]
        #vis fn #metadata_name() -> befu_bridge::CommandMetadata {
            befu_bridge::CommandMetadata {
                name: #command_name_str,
                description: #description_lit,
            }
        }
    };

    TokenStream::from(expanded)
}

struct RegisterInput {
    registry: Expr,
    commands: Punctuated<ExprPath, Token![,]>,
}

impl Parse for RegisterInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let registry = input.parse()?;
        input.parse::<Token![,]>()?;
        let commands = Punctuated::parse_terminated(input)?;
        Ok(RegisterInput { registry, commands })
    }
}

#[proc_macro]
pub fn register_commands(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RegisterInput);
    let registry = &input.registry;
    let mut expanded = quote! {};

    for path in input.commands.iter() {
        let mut wrapper_path = path.clone();
        let mut metadata_path = path.clone();

        if let Some(segment) = wrapper_path.path.segments.last_mut() {
            segment.ident = format_ident!("__befu_wrapper_{}", segment.ident);
        }
        if let Some(segment) = metadata_path.path.segments.last_mut() {
            segment.ident = format_ident!("__befu_metadata_{}", segment.ident);
        }

        expanded = quote! {
            #expanded
            #registry.register(#metadata_path(), #wrapper_path);
        };
    }

    TokenStream::from(expanded)
}
