use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn has_role(attr: TokenStream, item: TokenStream) -> TokenStream {
    let role = parse_macro_input!(attr as LitStr);
    let mut func = parse_macro_input!(item as ItemFn);

    func.sig.inputs.insert(
        0,
        parse_quote! { __auth_user: ::zitadel::actix::introspection::IntrospectedUser },
    );

    let original_block = func.block;
    func.block = parse_quote! {{
        if let Err(__deny) = ::macros_security_core::require_role(&__auth_user, #role) {
            return __deny;
        }
        #original_block
    }};

    quote!(#func).into()
}
