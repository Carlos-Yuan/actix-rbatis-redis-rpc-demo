use actix::impl_actix_config;
use proc_macro::TokenStream;
use rpcx::{impl_rpcx_client_call, impl_function_register};
use syn::{parse_macro_input, AttributeArgs, ItemFn,DeriveInput, ItemImpl};
use quote::{quote, ToTokens};

mod rpcx;
mod actix;
mod util;


#[proc_macro_attribute]
pub fn rpcx_call(args: TokenStream, imp: TokenStream) -> TokenStream {
    let arg = parse_macro_input!(args as AttributeArgs);
    let mut target_imp: ItemImpl = syn::parse(imp).unwrap();
    let stream = impl_rpcx_client_call(&mut target_imp, &arg);
    stream
}


#[proc_macro_attribute]
pub fn rpcx_register(args: TokenStream, imp: TokenStream) -> TokenStream {
    let arg = parse_macro_input!(args as AttributeArgs);
    let mut target_imp: ItemImpl = syn::parse(imp).unwrap();
    let stream = impl_function_register(&mut target_imp, &arg);
    stream
}

#[proc_macro_attribute]
pub fn actix_config(args: TokenStream, imp: TokenStream) -> TokenStream {
    let arg = parse_macro_input!(args as AttributeArgs);
    let ref mut target_impl: ItemImpl = syn::parse(imp).unwrap();
    let stream = impl_actix_config(target_impl, &arg);
    stream
}

#[proc_macro_derive(CsmNew)]
pub fn csm_new(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl #name{
            pub fn new()->Self{
                let mut client=rpcx::Client::new(CONF.rpc.address().as_str(),rpcx::Opt::default());
                let csm=#name{client};
                csm
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(DTOOrderByCheck)]
pub fn dto_order_by_check(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl #name{
            pub fn check_order_by(&self)->crate::model::Result<()>{
                if self.order_by.is_some(){
                    let split=self.order_by.as_ref().unwrap().split(",");
                    for item in split.into_iter(){
                        if item.contains("=")||item.contains(" or ")||item.contains(" union ")||item.contains(" select "){
                            return Err("SQL injection".into())
                        }
                    }
                }
                Ok(())
            }
        }
        
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
