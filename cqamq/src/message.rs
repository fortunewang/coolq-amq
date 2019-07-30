use serde::Serialize;

#[derive(Serialize)]
pub struct PrivateMessage {
    pub from: i64,
    pub message: String,
}

#[derive(Serialize)]
pub struct GroupMessage {
    pub group: i64,
    pub from: i64,
    pub message: String,
}

#[derive(Serialize)]
pub struct DiscussMessage {
    pub discuss: i64,
    pub from: i64,
    pub message: String,
}

#[derive(Serialize)]
pub struct SendPrivateMessageResponse {
    pub ok: bool,
}

#[derive(Serialize)]
pub struct SendGroupMessageResponse {
    pub ok: bool,
}

#[derive(Serialize)]
pub struct SendDiscussMessageResponse {
    pub ok: bool,
}
