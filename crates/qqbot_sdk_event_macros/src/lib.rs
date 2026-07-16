use heck::ToShoutySnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemEnum};

/// 为事件枚举生成无载荷 Kind 枚举及其单向转换实现。
#[proc_macro_attribute]
pub fn event_kind(_args: TokenStream, input: TokenStream) -> TokenStream {
    let event_enum = parse_macro_input!(input as ItemEnum);
    event_kind_impl(event_enum).into()
}

fn event_kind_impl(event_enum: ItemEnum) -> proc_macro2::TokenStream {
    let enum_ident = &event_enum.ident;
    let kind_ident = format_ident!("{}Kind", enum_ident);
    let variants: Vec<_> = event_enum
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect();

    let variants_vec: Vec<syn::Ident> = variants
        .clone()
        .iter()
        .map(|ident| format_ident!("{}_HANDLERS", ident.to_string().to_shouty_snake_case()))
        .collect();

    let ref_patterns: Vec<_> = event_enum
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            match &variant.fields {
                syn::Fields::Unit => quote!(#enum_ident::#ident),
                syn::Fields::Unnamed(fields) if fields.unnamed.is_empty() => {
                    quote!(#enum_ident::#ident())
                }
                syn::Fields::Unnamed(_) => quote!(#enum_ident::#ident(_)),
                syn::Fields::Named(_) => quote!(#enum_ident::#ident { .. }),
            }
        })
        .collect();

    quote! {
        #event_enum

        /// 与原事件枚举一一对应、不携带事件载荷的注册键。
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #kind_ident {
            #(#variants),*
        }


        #(
            pub static #variants_vec: ::std::sync::RwLock<Vec<qqbot_sdk_core::DynEventHandler>> =
                ::std::sync::RwLock::new(Vec::new());
        )*

        impl qqbot_sdk_core::KindRegistryKey for #kind_ident {
            fn get_writable_vec(&self) -> ::std::sync::RwLockWriteGuard<'static, Vec<qqbot_sdk_core::DynEventHandler>> {
                match self {
                    #(
                        Self::#variants => #variants_vec.write().unwrap(),
                    )*
                }
            }

            fn get_readable_vec(&self) -> ::std::vec::Vec<qqbot_sdk_core::DynEventHandler> {
                match self {
                    #(
                        Self::#variants => #variants_vec.read().unwrap().clone(),
                    )*
                }
            }
        }

        impl #enum_ident {
            pub fn to_kind(&self) -> #kind_ident {
                match self {
                    #(#ref_patterns => #kind_ident::#variants),*
                }
            }
        }

        impl From<&#enum_ident> for #kind_ident {
            fn from(event: &#enum_ident) -> Self {
                match event {
                    #(#ref_patterns => Self::#variants),*
                }
            }
        }

        impl From<#enum_ident> for #kind_ident {
            fn from(event: #enum_ident) -> Self {
                Self::from(&event)
            }
        }

    }
}
