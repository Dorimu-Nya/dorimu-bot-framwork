# qqbot_sdk

一个正在开发中的对接支持 QQ机器人 官方API Webhook 框架，目标是一键开箱，快速使用。

## 开始

照常创建一个空的Rust项目，然后使用 git submodule 添加本仓库

```sh
git submodule add https://github.com/Dorimu-Nya/dorimu-bot-sdk qqbot_sdk
```

随后，在 `Cargo.toml` 的 `dependencies` 区块 添加

```toml
qqbot_sdk = { path = "./qqbot_sdk" }
```

## 消息指令

命令不会由 `qqbot_sdk_app` 自动启用，需要显式创建并加载顶层 facade 提供的 `CommandPlugin`：

```rust
use qqbot_sdk::{AppConfig, CommandPlugin, ReplyingMessage};

let command_plugin = CommandPlugin::new()
    .with_command("/ping", || ReplyingMessage::Text("Pong!".to_string()));

let config = AppConfig::new().with_plugin(command_plugin);
```

`#[command(...)]` 注册的命令也会在 `CommandPlugin` 加载时一并收集。`qqbot_sdk_app` 本身不依赖 commands。

## 插件

插件通过 runtime 提供的注册器声明事件处理器，不直接依赖具体的 `App` 实现：

```rust
use qqbot_sdk::events::c2c::event_type::C2cEventTypeKind;
use qqbot_sdk::events::c2c::models::C2cMessage;
use qqbot_sdk::{AppConfig, Plugin, PluginRegistrar};

struct MyPlugin;

impl Plugin for MyPlugin {
    fn register(&self, registrar: &PluginRegistrar<'_>) {
        registrar.register_event_handler(
            C2cEventTypeKind::C2cMessageCreate,
            |message: C2cMessage| async move {
                println!("{:?}", message.content);
            },
        );
    }
}

let config = AppConfig::new().with_plugin(MyPlugin);
```

## 依赖注入
类似于actix/axum的状态注入，可以存储像数据库连接池等对象。
首先需要在初始化 AppConfig 时使用 `with_depend`

```rust
pub struct YourState;

let config = AppConfig::new();
//Your other config...
let config = config.with_depend(Depend::new(YourState));
```
可以在事件处理器中使用：
```rust
async fn on_message(message: C2cMessage, state: Depend<YourState>) {
    // Your biz logic...
}
```

普通事件参数从 webhook payload 提取，`Depend<T>` 从 runtime 依赖容器提取。

## 当前开发目标和进度

- [x] Webhook 事件的解析和处理函数
- [x] 事件处理函数注册和依赖注入
- [ ] open api 部分的代码指令提高和文档
- [x] 应用项目的启动参数的解析传递
- [ ] 其他事件的处理
- [ ] 独立的 `commands_app` 集成 crate

## 考虑/计划中/设想的未来目标
- 提供配置读取
- 其他的还没想好
