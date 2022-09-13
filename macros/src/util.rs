use std::ops::Index;

use quote::quote;
use quote::ToTokens;
use syn::ImplItemMethod;
use syn::Type;
use syn::{FnArg, ItemFn, Pat, ReturnType};

//find and check method return type
pub(crate) fn find_return_type<'a>(target_fn: &'a ItemFn, outer_ident_name: &str) -> Option<&'a syn::Type> {
    let mut return_ty = target_fn.sig.output.to_token_stream();
    if let ReturnType::Type(_, b)=&target_fn.sig.output{
        if let Type::Path(syn::TypePath { ref path, .. })=b.as_ref(){
            if let Some(seg) = path.segments.last() {
                if seg.ident == outer_ident_name  {
                    if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        ref args,
                        ..
                    }) = seg.arguments
                    {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                            return Some(inner_ty);
                        }
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn find_impl_method_return_type<'a>(target_fn: &'a ImplItemMethod, outer_ident_name: &str) -> Option<&'a syn::Type> {
    let mut return_ty = target_fn.sig.output.to_token_stream();
    if let ReturnType::Type(_, b)=&target_fn.sig.output{
        if let Type::Path(syn::TypePath { ref path, .. })=b.as_ref(){
            if let Some(seg) = path.segments.last() {
                if seg.ident == outer_ident_name  {
                    if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        ref args,
                        ..
                    }) = seg.arguments
                    {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                            return Some(inner_ty);
                        }
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn get_fn_args(target_fn: &ItemFn) -> Vec<Box<Pat>> {
    let mut fn_arg_name_vec = vec![];
    for arg in &target_fn.sig.inputs {
        match arg {
            FnArg::Typed(t) => {
                fn_arg_name_vec.push(t.pat.clone());
                //println!("arg_name {}", arg_name);
            }
            _ => {}
        }
    }
    fn_arg_name_vec
}

//find and check method return type
pub(crate) fn find_fn_body(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    //del todos
    let mut target_fn = target_fn.clone();
    let mut new_stmts = vec![];
    for x in &target_fn.block.stmts {
        let token = x
            .to_token_stream()
            .to_string()
            .replace("\n", "")
            .replace(" ", "");
        if token.eq("todo!()") || token.eq("unimplemented!()") || token.contains("impled!()") {
            //nothing to do
        } else {
            new_stmts.push(x.to_owned());
        }
    }
    target_fn.block.stmts = new_stmts;
    target_fn.block.to_token_stream()
}

pub(crate) fn is_rpcx_client_ref(ty_stream: &str) -> bool {
    if ty_stream.contains("Client") ||  ty_stream.contains("rpcx::Client") 
    {
        return true;
    }
    false
}
