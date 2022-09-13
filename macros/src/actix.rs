extern crate proc_macro;

use quote::quote;
use quote::ToTokens;
use syn::ImplItem;
use syn::{AttributeArgs, ItemImpl,Type};
use proc_macro::TokenStream;


///实现actix 注册
pub(crate) fn impl_actix_config(target_impl: &mut ItemImpl, args: &AttributeArgs) -> TokenStream {
    let mut controller_ident = "".to_token_stream();
    if let Type::Path(p)=target_impl.self_ty.as_ref(){
        controller_ident=p.path.segments.first().unwrap().ident.to_token_stream();
    }
    if controller_ident.is_empty(){
        let e=syn::Error::new_spanned(target_impl, "controller name not find").to_compile_error();
        return e.into();
    }
    let controller_name=controller_ident.to_string().replace("Controller", "");
    let mut methods:Vec<proc_macro2::TokenStream>=vec![];
    for item in target_impl.items.iter(){
        if let ImplItem::Method(m) = item{
            let func_ident=m.sig.ident.clone();
            let func_name=func_ident.to_string();
            methods.push(quote!{.service(web::resource(#func_name).route(web::post().to(#controller_ident::#func_ident)))});
        }
    }

    let ret = quote! {     
        pub fn config(cfg: &mut web::ServiceConfig) {
            // domain includes: /products/{product_id}/parts/{part_id}
            cfg.service(
                web::scope(format!("{}/{}",crate::CONF.prefix,#controller_name).as_str())
                #(#methods)*
            );
        }      
    };
    target_impl.items.push(syn::ImplItem::Verbatim(ret));
    target_impl.to_token_stream().into()
}