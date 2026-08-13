/// 对命令处理器支持的每一种参数数量调用一次给定宏。
///
/// 函数/闭包处理器和实现 [`crate::Command`] 的结构体必须共用这份列表，
/// 这样两种注册方式支持的参数数量才不会产生偏差。
macro_rules! for_each_command_arity {
    ($callback:ident) => {
        $callback!();
        $callback!(A1 => a1);
        $callback!(A1 => a1, A2 => a2);
        $callback!(A1 => a1, A2 => a2, A3 => a3);
        $callback!(A1 => a1, A2 => a2, A3 => a3, A4 => a4);
        $callback!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5);
        $callback!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6);
        $callback!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6, A7 => a7);
        $callback!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6, A7 => a7, A8 => a8);
    };
}

pub(crate) use for_each_command_arity;
