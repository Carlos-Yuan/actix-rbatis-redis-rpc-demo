extern crate proc_macro;

use crate::util::find_impl_method_return_type;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::ImplItem;
use syn::Type;
use syn::{AttributeArgs, FnArg, ItemImpl};

///实现rpcx client call
pub(crate) fn impl_rpcx_client_call(target_impl: &mut ItemImpl, args: &AttributeArgs) -> TokenStream {
    let mut csm_ident = "".to_token_stream();
    if let Type::Path(p)=target_impl.self_ty.as_ref(){
        csm_ident=p.path.segments.first().unwrap().ident.to_token_stream();
    }
    if csm_ident.is_empty(){
        let e=syn::Error::new_spanned(target_impl, "service name not find").to_compile_error();
        return e.into();
    }
    let csm_name=csm_ident.to_string().replace("Csm", "");
    let mut methods:Vec<proc_macro2::TokenStream>=vec![];
    for item in target_impl.items.iter(){
        if let ImplItem::Method(m) = item{
            
            let block=m.block.clone().into_token_stream();
            let func_ident=m.sig.ident.clone();
            let func_name=func_ident.to_string();
            let func_args_stream = m.sig.inputs.to_token_stream();
            let mut arg_ident = "".to_token_stream();
            if let FnArg::Typed(t) = m.sig.inputs.last().unwrap() {
                arg_ident = t.pat.to_token_stream();
            }
            let return_ty = find_impl_method_return_type(m, "Result");
            if return_ty.is_none() {
                let mut e = syn::Error::new_spanned(return_ty, "reply not empty").to_compile_error();
                e.extend(m.to_token_stream());
                return e.into();
            }
            let func_return_stream = m.sig.output.to_token_stream();
            methods.push(quote!{pub async fn #func_ident(#func_args_stream)#func_return_stream{
                #block
                let resp =self.client.send(#csm_name, #func_name, false, false,&std::collections::hash_map::HashMap::new(), #arg_ident).await;
                let reply:#return_ty = rpcx::get_result(resp, rpcx::SerializeType::MsgPack)?;
                Ok(reply)
            }});
        }
    }
    target_impl.items.clear();
    for m in methods.iter() {
        target_impl.items.push(syn::ImplItem::Verbatim(m.to_token_stream()));
    }
    // eprintln!("{:#?}",target_impl.to_token_stream().to_string().replace("\n", ""));
    target_impl.to_token_stream().into()
    // if let FnArg::Receiver(t) = target_fn.sig.inputs.first().unwrap() {
    //     if !t.mutability.is_some() {
    //         let mut e = syn::Error::new_spanned(target_fn, "first input must be mut self")
    //             .to_compile_error();
    //         e.extend(target_fn.to_token_stream());
    //         return e.into();
    //     }
    // }
    // let return_ty = find_return_type(target_fn, "Result");
    // if return_ty.is_none() {
    //     let mut e = syn::Error::new_spanned(return_ty, "reply not empty").to_compile_error();
    //     e.extend(target_fn.to_token_stream());
    //     return e.into();
    // }
    // let return_ty = return_ty.to_token_stream();
    // let func_name_ident = target_fn.sig.ident.to_token_stream();
    // let func_name = func_name_ident.to_string();
    // let mut arg_ident = "".to_token_stream();
    // let mut arg_name = String::new();
    // if let FnArg::Typed(t) = target_fn.sig.inputs.last().unwrap() {
    //     arg_ident = t.pat.to_token_stream();
    //     arg_name = arg_ident.to_string().trim_start_matches("mut ").to_string();
    // }
    // if arg_name.is_empty() {
    //     let mut e = syn::Error::new_spanned(target_fn, "dto arg not find").to_compile_error();
    //     e.extend(target_fn.to_token_stream());
    //     return e.into();
    // }
    // let func_args_stream = target_fn.sig.inputs.to_token_stream();
    // let fn_body = find_fn_body(target_fn);
    // let ret = quote! {
    //     pub async fn #func_name_ident(#func_args_stream)->Result<#return_ty>{
    //         #fn_body
    //         let resp:Option<rpcx::Result<#return_ty>> = self.client
    //             .call(
    //                 #func_name,
    //                 false,
    //                 &std::collections::hash_map::HashMap::new(),
    //                 #arg_ident,
    //             ).await;
    //             if !resp.is_none(){
    //                 match resp.unwrap() {
    //                     Ok(r) => { Ok(r)},
    //                     Err(err) =>{Err(err.to_string().into())},
    //                 }
    //             }else{
    //                 Err("call faild".into())
    //             }
    //     }
    // };
    // // eprintln!("{:?}",ret.to_string().replace("\n", ""));
    // ret.into()
}

//rpcx服务方法注册
pub(crate) fn impl_function_register(
    target_impl: &mut ItemImpl,
    args: &AttributeArgs,
) -> TokenStream {
    let mut pvd_ident = "".to_token_stream();
    if let Type::Path(p) = target_impl.self_ty.as_ref() {
        pvd_ident = p.path.segments.first().unwrap().ident.to_token_stream();
    }
    if pvd_ident.is_empty() {
        let e = syn::Error::new_spanned(target_impl, "service name not find").to_compile_error();
        return e.into();
    }
    let pvd_name = pvd_ident.to_string().replace("Pvd", "");
    let mut methods: Vec<proc_macro2::TokenStream> = vec![];
    for item in target_impl.items.iter() {
        if let ImplItem::Method(m) = item {
            let func_ident = m.sig.ident.clone();
            let func_name = func_ident.to_string();
            let mut arg_type = "".to_token_stream();
            if let FnArg::Typed(pt) = m.sig.inputs.first().unwrap() {
                if let Type::Path(syn::TypePath { ref path, .. })=pt.ty.as_ref(){
                    if let Some(seg) = path.segments.first() {
                        arg_type=seg.ident.to_token_stream();
                    }
                }
            }
            if arg_type.is_empty() {
                let e = syn::Error::new_spanned(target_impl, "function args not find").to_compile_error();
                return e.into();
            }
            methods.push(quote! {
                RPC.lock().unwrap().get_mut().register_fn(
                    #pvd_name.into(),
                    #func_name.into(),
                    "".to_string(),
                    |x, st| {
                        let mut args: #arg_type = Default::default();
                        args.from_slice(st, x)?;
                        let res = RT.block_on(#pvd_ident::#func_ident(args));
                        match res {
                            Ok(res) => res.into_bytes(st),
                            Err(e) => Err(rpcx::Error::new(rpcx::ErrorKind::Server, e.to_string())),
                        }
                    },
                );
            });
        }
    }

    let ret = quote! {
        pub fn register() {
            #(#methods)*
        }
    };
    target_impl.items.push(syn::ImplItem::Verbatim(ret));
    // eprintln!("{:?}",target_impl.into_token_stream().to_string().replace("\n", ""));
    target_impl.to_token_stream().into()
}
