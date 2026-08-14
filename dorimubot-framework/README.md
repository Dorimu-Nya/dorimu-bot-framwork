# dorimubot-framework

基于 [qqbot-rust-sdk](https://github.com/Dorimu-Nya/qqbot-rust-sdk) 开发的QQ机器人开发框架。

## 快速开始

参考 https://github.com/Dorimu-Nya/bot-quickstart-template

## 注册事件处理器

事件通过框架提供的标记类型注册。标记类型会在编译期绑定事件和它的 payload：

示例 1（异步闭包）：
```rust
use dorimubot_framework_core::{events, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::c2c::models::C2cMessage;

fn main() {
    let app = QQBot::new(QQBotConfig::new());
    app.register_event_handler(
        events::c2c::C2cMessageCreate,
        move |message: C2cMessage| async move {
            println!("收到消息:{:?}", message);
        },
    );
}
```

示例 2（同步函数）：
```rust
use dorimubot_framework_core::{events, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::group::models::GroupMessage;

fn main() {
    let app = QQBot::new(QQBotConfig::new());
    app.register_event_handler(
        events::group::GroupAtMessageCreate,
        group_message_handler,
    );
}

fn group_message_handler(message: GroupMessage) {
    println!("group_message_handler: {:?}", message);
}
```

同步和异步 handler 均受支持。handler 可以接收 0～8 个 owned 参数；对每个参数
`Arg`，该事件的 payload 都必须满足 `Payload: Into<Arg>`。借用参数（例如
`&C2cMessage`）不受支持；需要共享的状态或 API 客户端可以由闭包捕获。

## 注册指令
需要先启用 feature `commands`

Example 1:
```rust
fn main() {
    let app = QQBot::new(QQBotConfig::new());
    let command_plugin = CommandPlugin::new()
        .with_command("/ping", || ReplyingMessage::Text("Pong!".to_string()));
    command_plugin.register(&app);
}
```

Example 2:
需要启用 feature `commands-macros`
```rust
fn main() {
    let app = QQBot::new(QQBotConfig::new());
    let command_plugin = CommandPlugin::new();
    command_plugin.register(&app);
}

#[command("/ping")]
fn ping() -> ReplyingMessage {
    Text(String::from("Pong!"))
}
```

Example 3:
有状态命令可以实现 `Command` 后直接传给 `with_command`。`Args` 使用元组表示参数列表，
支持 0～8 个 `FromCommonMessage` 参数，并会和函数/闭包处理器一样自动从消息中提取：
```rust
use dorimubot_framework::dorimubot_commands::{Command, CommandPlugin, ReplyingMessage};

struct CountingCommand {
    calls: usize,
}

impl Command for CountingCommand {
    type Args = (String, Option<Vec<String>>);
    type Output = ReplyingMessage;

    fn handle(&mut self, (content, words): Self::Args) -> Self::Output {
        self.calls += 1;
        ReplyingMessage::Text(format!(
            "{content}; {} words; called {} times",
            words.map_or(0, |words| words.len()),
            self.calls,
        ))
    }
}

fn command_plugin() -> CommandPlugin {
    CommandPlugin::new().with_command("/count", CountingCommand { calls: 0 })
}
```

异步且需要修改自身状态的命令可实现 `AsyncCommand`。零参数使用 `type Args = ()`，
单参数使用 `(A1,)`，多个参数使用 `(A1, A2, ...)`。

## 当前开发目标和进度

- [x] Webhook 事件的解析和处理函数
- [x] 事件处理函数注册
- [ ] open api 部分的代码指令提高和文档
- [x] 应用项目的启动参数的解析传递
- [x] 其他事件的处理
- [ ] 重新实现先前移除的IoC容器
- [ ] 实现拦截器/后处理器等
