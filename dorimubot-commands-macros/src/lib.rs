mod command;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Command 宏
///
/// 用于定义命令处理函数。宏会自动：
/// 1. 扫描函数参数并从消息中提取参数
/// 2. 注册命令到全局命令表
///
/// # 用法
/// ```ignore
/// #[command("/ping")]
/// fn ping() -> ReplyingMessage {
///     ReplyingMessage::Text("Pong!".to_string())
/// }
/// ```
#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as command::CommandArgs);
    let func = parse_macro_input!(input as syn::ItemFn);

    command::command_impl(args, func).into()
}
