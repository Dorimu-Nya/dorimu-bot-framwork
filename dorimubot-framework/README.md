# dorimubot-framework

基于 [qqbot-rust-sdk](https://github.com/Dorimu-Nya/qqbot-rust-sdk) 开发的QQ机器人开发框架。

## 快速开始

参考 https://github.com/Dorimu-Nya/bot-quickstart-template

## 注册事件处理器

Example 1:
```rust
fn main() {
    let app = QQBot::new(QQBotConfig::new());
    app.register_event_handler(
        C2cEventKind::C2cMessageCreate,
        move |_message: C2cMessage| {
            println!("收到消息:{:?}", _message);
        },
    );
}
```
Example 2:
```rust
fn main() {
    let app = QQBot::new(QQBotConfig::new());
    app.register_event_handler(GroupEventKind::GroupAtMessageCreate, group_message_handler)
}

fn group_message_handler(message: GroupMessage) {
    println!("group_message_handler: {:?}", message);
}
```

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
## 当前开发目标和进度

- [x] Webhook 事件的解析和处理函数
- [x] 事件处理函数注册
- [ ] open api 部分的代码指令提高和文档
- [x] 应用项目的启动参数的解析传递
- [x] 其他事件的处理
- [ ] 重新实现先前移除的IoC容器
- [ ] 实现拦截器/后处理器等
