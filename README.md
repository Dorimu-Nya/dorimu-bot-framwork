# dorimubot-framework

一个正在开发中的对接支持 QQ机器人 官方API Webhook 框架，目标是一键开箱，快速使用。

## 开始

照常创建一个空的Rust项目，然后使用 git submodule 添加本仓库

```sh
git submodule add https://github.com/Dorimu-Nya/dorimubot-framework dorimubot-framework
```

随后，在 `Cargo.toml` 的 `dependencies` 区块 添加

```toml
dorimubot-framework = { path = "./dorimubot-framework/dorimubot-framework" }
dorimubot_commands = { path = "./dorimubot-framework/dorimubot_commands" }
```

## 消息指令

命令功能位于独立的 `dorimubot_commands` crate，需要显式注册到应用：

```rust
use dorimubot_commands::{CommandPlugin, ReplyingMessage};
use dorimubot_framework::{AppConfig, QQBotApp};

let command_plugin = CommandPlugin::new()
    .with_command("/ping", || ReplyingMessage::Text("Pong!".to_string()));

let app = QQBotApp::new(AppConfig::new());
command_plugin.register(&app);
```

`#[command(...)]` 注册的命令也会在 `CommandPlugin::register` 时一并收集。依赖方向保持为 `dorimubot_commands -> dorimubot-framework`，framework 不反向依赖 commands。

需要共享状态时，直接通过 `Arc` 和闭包显式捕获：

```rust
let state = Arc::new(YourState::new());
let handler_state = Arc::clone(&state);
app.register_event_handler(C2cEventKind::C2cMessageCreate, move |message: C2cMessage| {
    let state = Arc::clone(&handler_state);
    async move {
        state.handle(message).await;
    }
});
```

## 当前开发目标和进度

- [x] Webhook 事件的解析和处理函数
- [x] 事件处理函数注册
- [ ] open api 部分的代码指令提高和文档
- [x] 应用项目的启动参数的解析传递
- [ ] 其他事件的处理
- [ ] 独立的 `commands_app` 集成 crate

## 考虑/计划中/设想的未来目标
- 提供配置读取
- 其他的还没想好
