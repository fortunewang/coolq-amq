mod config;
mod message;
mod client;
use self::client::AMQPClient;

use std::os::raw::c_char;
use cqrs::gb18030_decode;

static APP_NAME: &[u8] = b"9,me.cqp.fortunewang.amq\0";

static mut G_AUTH_CODE: i32 = -1;
static mut G_CLIENT: Option<AMQPClient> = None;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn AppInfo() -> *const c_char {
	return APP_NAME.as_ptr() as *const i8;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn Initialize(auth_code: i32) -> i32 {
    unsafe { G_AUTH_CODE = auth_code; }
	return 0;
}

#[no_mangle]
pub extern "stdcall" fn app_on_startup() -> i32 { 0 }

#[no_mangle]
pub extern "stdcall" fn app_on_exit() -> i32 { 0 }

#[no_mangle]
pub extern "stdcall" fn app_on_enabled() -> i32 {
    let mut client = AMQPClient::new(unsafe{G_AUTH_CODE}).unwrap();
    client.start();
    unsafe {
        G_CLIENT = Some(client);
    }
    0
}

#[no_mangle]
pub extern "stdcall" fn app_on_disable() -> i32 { 0 }

#[no_mangle]
pub extern "stdcall" fn app_on_private_message(_subtype: i32, _msgid: i32,
    from_qq: i64, msg: *const c_char, _font: i32) -> i32 {
    if let Some(client) = unsafe { G_CLIENT.as_ref() } {
        let msg = crate::message::PrivateMessage {
            from: from_qq,
            message: unsafe { gb18030_decode(msg).unwrap() },
        };
        client.send_message("private", serde_json::to_vec(&msg).unwrap());
    }
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_group_message(_subtype: i32, _msgid: i32,
    from_group: i64, from_qq: i64, _from_anonymous: *const c_char,
    msg: *const c_char, _font: i32) -> i32 {
    if let Some(client) = unsafe { G_CLIENT.as_ref() } {
        let msg = crate::message::GroupMessage {
            group: from_group,
            from: from_qq,
            message: unsafe { gb18030_decode(msg).unwrap() },
        };
        client.send_message("group", serde_json::to_vec(&msg).unwrap());
    }
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_discuss_message(_subtype: i32, _msgid: i32,
    from_discuss: i64, from_qq: i64,
    msg: *const c_char, _font: i32) -> i32 {
    if let Some(client) = unsafe { G_CLIENT.as_ref() } {
        let msg = crate::message::DiscussMessage {
            discuss: from_discuss,
            from: from_qq,
            message: unsafe { gb18030_decode(msg).unwrap() },
        };
        client.send_message("discuss", serde_json::to_vec(&msg).unwrap());
    }
    return cqrs::EventResultCode::Ignore as i32;
}
