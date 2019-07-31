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
pub struct GroupAdminChanged {
    pub group: i64,
    pub operand: i64,
    pub set: bool,
}

#[derive(Serialize)]
pub struct GroupMemberIncrease {
    pub group: i64,
    pub from: i64,
    pub operator: i64,
    pub invited: bool,
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
