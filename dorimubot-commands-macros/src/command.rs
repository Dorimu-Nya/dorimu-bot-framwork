use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{parse::Parse, parse::ParseStream, FnArg, ItemFn, LitStr, Result};

/// Command 宏的参数
///
/// 只包含命令前缀，不需要指定状态类型
pub struct CommandArgs {
    /// 命令前缀（如 "/ping"）
    pub prefix: LitStr,
}

impl Parse for CommandArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let prefix: LitStr = input.parse()?;
        Ok(CommandArgs { prefix })
    }
}

/// Command 宏的实现函数
///
/// 这个函数会：
/// 1. 分析函数参数
/// 2. 生成消息参数提取包装函数
/// 3. 生成 inventory 注册代码
///
/// # 参数
/// * `args` - 宏参数（命令前缀）
/// * `func` - 被标注的函数
///
/// # 返回
/// 生成的完整代码
pub fn command_impl(args: CommandArgs, func: ItemFn) -> TokenStream {
    let prefix = args.prefix;
    let fn_name = &func.sig.ident;
    // 生成包装函数的名称
    let wrapper_name = format_ident!("__dorimubot_framework_command_wrapper_{}", fn_name);
    let is_async = func.sig.asyncness.is_some();
    let commands = commands_crate_path();

    let mut param_extractions = Vec::new();
    let mut call_args = Vec::new();

    for (index, arg) in func.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(_) => {
                panic!("Methods with self are not supported");
            }
            FnArg::Typed(typed) => {
                let ty = &typed.ty;
                let arg_name = format_ident!("__arg_{}", index);
                param_extractions.push(quote! {
                    let #arg_name: #ty = <#ty as #commands::FromCommonMessage<'_>>::from(__message);
                });
                call_args.push(quote! { #arg_name });
            }
        }
    }

    // 判断是否异步生成对应封装
    let invoke = if is_async {
        quote! {
            let result = #fn_name(#(#call_args),*).await;
        }
    } else {
        quote! {
            let result = #fn_name(#(#call_args),*);
        }
    };

    // 生成最终代码
    quote! {
        // 保留原函数定义
        #func

        // 在匿名常量中生成包装代码，避免命名冲突
        const _: () = {
            // 包装函数：接收消息并返回 Future
            fn #wrapper_name<'a>(
                __message: &'a dyn #commands::CommonMessage,
            ) -> #commands::CommandHandleFuture<'a> {
                ::std::boxed::Box::pin(async move {
                    #(#param_extractions)*
                    #invoke
                    // 将结果转换为统一的输出格式
                    #commands::CommandOutput::into_output(result)
                })
            }

            // 使用 inventory 注册命令
            #commands::inventory::submit! {
                #commands::CommandDef {
                    prefix: #prefix,
                    handler: #wrapper_name
                }
            }
        };
    }
}

fn commands_crate_path() -> TokenStream {
    match crate_name("dorimubot-commands") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let name = format_ident!("{}", name);
            quote!(::#name)
        }
        Err(_) => match crate_name("dorimubot-framework") {
            Ok(FoundCrate::Itself) => quote!(crate::commands),
            Ok(FoundCrate::Name(name)) => {
                let name = format_ident!("{}", name);
                quote!(::#name::dorimubot_commands)
            }
            Err(error) => panic!(
                "#[command] requires either dorimubot-commands or dorimubot-framework: {error}"
            ),
        },
    }
}
