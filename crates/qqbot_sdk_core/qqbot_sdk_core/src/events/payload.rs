use super::event::Event;
use super::opcode::{DispatchOp, HttpCallbackAckOp, WebhookAddressVerifyOp};
use crate::events::validation::ValidationRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookPayload {
    // opcode=0
    /// 服务端进行消息推送
    Dispatch(DispatchPayload),
    // opcode = 12
    /// http 回调模式的回包
    HttpCallbackAck(HttpCallbackAckPayload),
    // opcode=13
    /// 开放平台对机器人服务端进行验证
    WebhookAddressVerify(WebhookAddressVerifyPayload),
}

/// opcode为0时，服务端进行消息推送的消息对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchPayload {
    /// 事件id
    pub id: Option<String>,
    ///  下行消息都会有一个序列号，标识消息的唯一性，客户端需要再发送心跳的时候，携带客户端收到的最新的s
    pub s: Option<u64>,
    /// 代表事件类型
    pub op: DispatchOp,
    /// 代表事件内容，不同事件类型的事件内容格式都不同，请注意识别。主要用在op为 0 Dispatch 的时候
    #[serde(flatten)]
    pub event: Event,
}

/// 从完整事件载荷中提取事件处理器参数。
///
/// 返回 `None` 表示当前载荷不包含该类型；事件处理器会跳过本次调用。
pub trait FromDispatchPayload: Sized {
    fn from(req: &DispatchPayload) -> Option<Self>;
}

impl FromDispatchPayload for DispatchPayload {
    fn from(req: &DispatchPayload) -> Option<Self> {
        Some(req.clone())
    }
}

/// opcode为12时，http 回调模式的回包
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpCallbackAckPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: HttpCallbackAckOp,
}

/// opcode为13时，开放平台对机器人服务端进行验证时的消息对象
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookAddressVerifyPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: WebhookAddressVerifyOp,
    pub d: ValidationRequest,
}
