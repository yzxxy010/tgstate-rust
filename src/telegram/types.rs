use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelegramResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub channel_post: Option<Message>,
    // getUpdates 订阅了 edited_* 事件（见 bot_polling 的 allowed_updates），
    // 但目前不对其做处理，仅保留字段以完整映射 API 载荷。
    #[allow(dead_code)]
    pub edited_message: Option<Message>,
    #[allow(dead_code)]
    pub edited_channel_post: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub chat: Chat,
    pub text: Option<String>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub date: Option<i64>,
    pub reply_to_message: Option<Box<Message>>,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_size: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TelegramFile {
    pub file_id: String,
    pub file_path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BotUser {
    pub username: Option<String>,
}
