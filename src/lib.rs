use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Attribute, Data::Struct, DeriveInput, GenericParam};
#[proc_macro_derive(AsyncTryClone)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_data = match ast.data {
        Struct(s) => s,
        _ => {
            panic!("currently only works for structs");
        }
    };
    let struct_ident = ast.ident;
    let named_fields = match struct_data.fields {
        syn::Fields::Named(f) => f.named,
        _ => {
            panic!("currently only works for named fields");
        }
    };
    let mut generics = ast.generics;
    for p in &mut generics.params {
        if let GenericParam::Type(ref mut t) = *p {
            t.bounds.push(parse_quote!(AsyncTryClone));
            t.bounds.push(parse_quote!(::std::marker::Send));
            t.bounds.push(parse_quote!(::std::marker::Sync));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let async_trait: Attribute = parse_quote!(#[async_trait]);
    let prefix_fields = named_fields.iter().map(|f| {
        let field_name = &f.ident;
        let prefix_field_name = format_ident!("_{}", field_name.as_ref().unwrap());
        quote! {
            #prefix_field_name
        }
    });
    let fields_asyncs = named_fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            self.#field_name.async_try_clone()
        }
    });
    let fields_assign = named_fields.iter().map(|f| {
        let field_name = &f.ident;
        let prefix_field_name = format_ident!("_{}", field_name.as_ref().unwrap());
        quote! {
            #field_name: #prefix_field_name
        }
    });
    let trait_fn = quote! {
        async fn async_try_clone(&self) -> Result<Self,Error>{
            let result = tokio::try_join!(
                #(#fields_asyncs),*
            );
            match result{
                Ok((#(#prefix_fields),*))=> {
                    Ok(
                        Self{
                            #(#fields_assign),*
                        }
                    )

                }
                Err(e) => {
                    Err(e)
                }
            }
        }
    };
    let output = quote! {
        #async_trait
        impl #impl_generics AsyncTryClone for #struct_ident #ty_generics #where_clause {
            #trait_fn
        }
    };
    TokenStream::from(output)
}
