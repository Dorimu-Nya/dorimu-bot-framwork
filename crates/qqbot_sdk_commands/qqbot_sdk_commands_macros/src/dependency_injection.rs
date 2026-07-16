use quote::quote;
use syn::{GenericArgument, PathArguments, Type, TypePath};

/// 当参数是 `Depend<T>` 时返回其中的类型。
pub fn extract_depend_inner_type(ty: &Type) -> Option<Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };
    if !is_depend_path(type_path) {
        return None;
    }
    let segment = type_path.path.segments.last()?;
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    let GenericArgument::Type(inner) = args.args.first()? else {
        return None;
    };
    Some(inner.clone())
}

/// 生成从运行时解析器中解析 `Depend<T>` 的表达式。
pub fn quoting_depend_param(inner_type: Type) -> proc_macro2::TokenStream {
    quote! {
        qqbot_sdk::Depend::<#inner_type>::from_provider(__dependencies)
    }
}

fn is_depend_path(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|segment| segment.ident == "Depend")
        .unwrap_or(false)
}
