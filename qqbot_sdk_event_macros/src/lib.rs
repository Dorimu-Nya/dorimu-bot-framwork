use heck::ToShoutySnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemEnum};
/// 为事件枚举生成 Kind 对应的处理器容器。
///
/// 参数是顶层 `Event` 对应的变体路径，例如 `Event::C2cEventType`。
// #[proc_macro_attribute]
// pub fn event_handlers(args: TokenStream, input: TokenStream) -> TokenStream {
//     let args = parse_macro_input!(args as EventHandlersArgs);
//     let event_enum = parse_macro_input!(input as ItemEnum);
//     event_handlers_impl(args.parent_variant, event_enum).into()
// }

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
            pub static #variants_vec: ::std::sync::RwLock<Vec<qqbot_sdk::DynEventHandler>> =
                ::std::sync::RwLock::new(Vec::new());
        )*

        impl qqbot_sdk::KindRegistryKey for #kind_ident {
            fn get_writable_vec(&self) -> ::std::sync::RwLockWriteGuard<'static, Vec<qqbot_sdk::DynEventHandler>> {
                match self {
                    #(
                        Self::#variants => #variants_vec.write().unwrap(),
                    )*
                }
            }

            fn get_readable_vec(&self) -> ::std::vec::Vec<qqbot_sdk::DynEventHandler> {
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

// fn event_handlers_impl(
//     parent_variant: Path,
//     event_enum: ItemEnum,
// ) -> proc_macro2::TokenStream {
//     let enum_ident = &event_enum.ident;
//     let kind_ident = format_ident!("{}Kind", enum_ident);
//     let handlers_ident = format_ident!("{}Handlers", enum_ident);
//     let container_field = format_ident!("{}", camel_to_snake(&enum_ident.to_string()));

//     quote! {
//         #event_enum

//         impl crate::app::events::EventKind for #kind_ident {
//             fn register(self, handlers: &mut crate::app::events::EventHandlers, handler: crate::app::events::DynEventHandler) {
//                 handlers.#container_field.register(self, handler);
//             }
//         }

//         /// 此事件枚举对应的处理器容器，按 Kind 分组保存处理器。
//         #[derive(Clone, Default)]
//         pub(crate) struct #handlers_ident {
//             handlers: ::std::collections::HashMap<#kind_ident, ::std::vec::Vec<crate::app::events::DynEventHandler>>,
//         }

//         impl #handlers_ident {
//             pub(crate) fn register(&mut self, kind: #kind_ident, handler: crate::app::events::DynEventHandler) {
//                 self.handlers.entry(kind).or_default().push(handler);
//             }

//             pub(crate) async fn dispatch(
//                 &self,
//                 event: &crate::events::event::Event,
//                 payload: &crate::events::payload::DispatchPayload,
//             ) {
//                 if let #parent_variant(event) = event {
//                     let kind = #kind_ident::from(event);
//                     if let Some(handlers) = self.handlers.get(&kind) {
//                         for handler in handlers {
//                             handler(payload).await;
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// fn camel_to_snake(value: &str) -> String {
//     let mut result = String::with_capacity(value.len());
//     let mut previous_is_lower_or_digit = false;

//     for character in value.chars() {
//         if character.is_uppercase() && previous_is_lower_or_digit {
//             result.push('_');
//         }
//         result.extend(character.to_lowercase());
//         previous_is_lower_or_digit = character.is_lowercase() || character.is_ascii_digit();
//     }

//     result
// }
