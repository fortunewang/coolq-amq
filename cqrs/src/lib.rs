mod utils;
pub use utils::{gb18030_encode, gb18030_decode};

use std::io;
use std::os::raw::c_char;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

pub type CQBool = i32;

static MODULE_NAME: &[u8] = b"CQP\0";
static FN_NAME_SEND_PRIVATE_MSG: &[u8] = b"CQ_sendPrivateMsg\0";
static FN_NAME_SEND_GROUP_MSG: &[u8] = b"CQ_sendGroupMsg\0";
static FN_NAME_SEND_DISCUSS_MSG: &[u8] = b"CQ_sendDiscussMsg\0";
static FN_NAME_DELETE_MSG: &[u8] = b"CQ_deleteMsg\0";
static FN_NAME_SEND_LIKE: &[u8] = b"CQ_sendLike\0";
static FN_NAME_SEND_LIKE_V2: &[u8] = b"CQ_sendLikeV2\0";
static FN_NAME_SET_GROUP_KICK: &[u8] = b"CQ_setGroupKick\0";
static FN_NAME_SET_GROUP_BAN: &[u8] = b"CQ_setGroupBan\0";
static FN_NAME_SET_GROUP_ADMIN: &[u8] = b"CQ_setGroupAdmin\0";
static FN_NAME_SET_GROUP_WHOLE_BAN: &[u8] = b"CQ_setGroupWholeBan\0";
static FN_NAME_SET_GROUP_ANONYMOUS_BAN: &[u8] = b"CQ_setGroupAnonymousBan\0";
static FN_NAME_SET_GROUP_ANONYMOUS: &[u8] = b"CQ_setGroupAnonymous\0";
static FN_NAME_SET_GROUP_CARD: &[u8] = b"CQ_setGroupCard\0";
static FN_NAME_SET_GROUP_LEAVE: &[u8] = b"CQ_setGroupLeave\0";
static FN_NAME_SET_GROUP_SPECIAL_TITLE: &[u8] = b"CQ_setGroupSpecialTitle\0";
static FN_NAME_SET_DISCUSS_LEAVE: &[u8] = b"CQ_setDiscussLeave\0";
static FN_NAME_SET_FRIEND_ADD_REQUEST: &[u8] = b"CQ_setFriendAddRequest\0";
static FN_NAME_SET_GROUP_ADD_REQUEST: &[u8] = b"CQ_setGroupAddRequest\0";
static FN_NAME_SET_GROUP_ADD_REQUEST_V2: &[u8] = b"CQ_setGroupAddRequestV2\0";
static FN_NAME_GET_GROUP_MEMBER_INFO_V2: &[u8] = b"CQ_getGroupMemberInfoV2\0";
static FN_NAME_GET_STRANGER_INFO: &[u8] = b"CQ_getStrangerInfo\0";
static FN_NAME_GET_GROUP_LIST: &[u8] = b"CQ_getGroupList\0";
static FN_NAME_GET_GROUP_MEMBER_LIST: &[u8] = b"CQ_getGroupMemberList\0";
static FN_NAME_ADD_LOG: &[u8] = b"CQ_addLog\0";
static FN_NAME_GET_COOKIES: &[u8] = b"CQ_getCookies\0";
static FN_NAME_GET_CSRF_TOKEN: &[u8] = b"CQ_getCsrfToken\0";
static FN_NAME_GET_LOGIN_QQ: &[u8] = b"CQ_getLoginQQ\0";
static FN_NAME_GET_LOGIN_NICK: &[u8] = b"CQ_getLoginNick\0";
static FN_NAME_GET_APP_DIRECTORY: &[u8] = b"CQ_getAppDirectory\0";
static FN_NAME_SET_FATAL: &[u8] = b"CQ_setFatal\0";
static FN_NAME_GET_RECORD: &[u8] = b"CQ_getRecord\0";
static FN_NAME_GET_RECORD_V2: &[u8] = b"CQ_getRecordV2\0";
static FN_NAME_GET_IMAGE: &[u8] = b"CQ_getImage\0";
static FN_NAME_CAN_SEND_IMAGE: &[u8] = b"CQ_canSendImage\0";
static FN_NAME_CAN_SEND_RECORD: &[u8] = b"CQ_canSendRecord\0";

#[inline]
const fn as_cstr(slice: &[u8]) -> *const i8 {
    slice.as_ptr() as *const i8
}

macro_rules! get_func {   
    ( $cqp: expr, $name: expr ) => (std::mem::transmute(GetProcAddress($cqp, as_cstr($name))))
}

pub enum EventResultCode {
    Ignore = 0,
    Block = 1,
}

pub enum RequestResultCode {
    Allow = 1,
    Deny = 2,
}

pub enum GroupRequestType {
    Add = 1,
    Invite = 2,
}

pub enum LogLevel {
    Debug = 0,
    Info = 10,
    InfoSuccess = 11,
    InfoRecv = 12,
    InfoSend = 13,
    Warning = 20,
    Error = 30,
    Fatal = 40,
}

pub struct API {
    fn_send_private_msg: extern "stdcall" fn(i32, i64, *const c_char) -> i32,
    fn_send_group_msg: extern "stdcall" fn(i32, i64, *const c_char) -> i32,
    fn_send_discuss_msg: extern "stdcall" fn(i32, i64, *const c_char) -> i32,
    fn_delete_msg: extern "stdcall" fn(i32, i64) -> i32,
    fn_send_like: extern "stdcall" fn(i32, i64) -> i32,
    fn_send_like_v2: extern "stdcall" fn(i32, i64, i32) -> i32,
    fn_set_group_kick: extern "stdcall" fn(i32, i64, i64, CQBool) -> i32,
    fn_set_group_ban: extern "stdcall" fn(i32, i64, i64, i64) -> i32,
    fn_set_group_admin: extern "stdcall" fn(i32, i64, i64, CQBool) -> i32,
    fn_set_group_whole_ban: extern "stdcall" fn(i32, i64, CQBool) -> i32,
    fn_set_group_anonymous_ban: extern "stdcall"
        fn(i32, i64, *const c_char, i64) -> i32,
    fn_set_group_anonymous: extern "stdcall" fn(i32, i64, CQBool) -> i32,
    fn_set_group_card: extern "stdcall"
        fn(i32, i64, i64, *const c_char) -> i32,
    fn_set_group_leave: extern "stdcall" fn(i32, i64, CQBool) -> i32,
    fn_set_group_special_title: extern "stdcall"
        fn(i32, i64, i64, *const c_char, i64) -> i32,
    fn_set_discuss_leave: extern "stdcall" fn(i32, i64) -> i32,
    fn_set_friend_add_request: extern "stdcall"
        fn(i32, *const c_char, i32, *const c_char) -> i32,
    fn_set_group_add_request: extern "stdcall"
        fn(i32, *const c_char, i32, i32) -> i32,
    fn_set_group_add_request_v2: extern "stdcall"
        fn(i32, *const c_char, i32, i32, *const c_char) -> i32,
    fn_get_group_member_info_v2: extern "stdcall"
        fn(i32, i64, i64, CQBool) -> *const c_char,
    fn_get_stranger_info: extern "stdcall" fn(i32, i64, CQBool) -> *const c_char,
    fn_get_group_list: extern "stdcall" fn(i32) -> *const c_char,
    fn_get_group_member_list: extern "stdcall" fn(i32, i64) -> *const c_char,
    fn_add_log: extern "stdcall"
        fn(i32, i32, *const c_char, *const c_char) -> i32,
    fn_get_cookies: extern "stdcall" fn(i32) -> *const c_char,
    fn_get_csrf_token: extern "stdcall" fn(i32) -> i32,
    fn_get_login_qq: extern "stdcall" fn(i32) -> i64,
    fn_get_login_nick: extern "stdcall" fn(i32) -> *const c_char,
    fn_get_app_directory: extern "stdcall" fn(i32) -> *const c_char,
    fn_set_fatal: extern "stdcall" fn(i32, *const c_char) -> i32,
    fn_get_record: extern "stdcall" fn(i32, *const c_char, *const c_char) -> *const c_char,
    fn_get_record_v2: extern "stdcall" fn(i32, *const c_char, *const c_char) -> *const c_char,
    fn_get_image: extern "stdcall" fn(i32, *const c_char) -> *const c_char,
    fn_can_send_image: extern "stdcall" fn(i32) -> CQBool,
    fn_can_send_record: extern "stdcall" fn(i32) -> CQBool,
}

impl API {

    pub fn new() -> io::Result<API> {
        let cqp = unsafe { GetModuleHandleA(as_cstr(MODULE_NAME)) };
        if cqp.is_null() {
            return Err(io::Error::last_os_error());
        }
        let api = unsafe { API {
            fn_send_private_msg: get_func!(cqp, FN_NAME_SEND_PRIVATE_MSG),
            fn_send_group_msg: get_func!(cqp, FN_NAME_SEND_GROUP_MSG),
            fn_send_discuss_msg: get_func!(cqp, FN_NAME_SEND_DISCUSS_MSG),
            fn_delete_msg: get_func!(cqp, FN_NAME_DELETE_MSG),
            fn_send_like: get_func!(cqp, FN_NAME_SEND_LIKE),
            fn_send_like_v2: get_func!(cqp, FN_NAME_SEND_LIKE_V2),
            fn_set_group_kick: get_func!(cqp, FN_NAME_SET_GROUP_KICK),
            fn_set_group_ban: get_func!(cqp, FN_NAME_SET_GROUP_BAN),
            fn_set_group_admin: get_func!(cqp, FN_NAME_SET_GROUP_ADMIN),
            fn_set_group_whole_ban: get_func!(cqp, FN_NAME_SET_GROUP_WHOLE_BAN),
            fn_set_group_anonymous_ban: get_func!(cqp, FN_NAME_SET_GROUP_ANONYMOUS_BAN),
            fn_set_group_anonymous: get_func!(cqp, FN_NAME_SET_GROUP_ANONYMOUS),
            fn_set_group_card: get_func!(cqp, FN_NAME_SET_GROUP_CARD),
            fn_set_group_leave: get_func!(cqp, FN_NAME_SET_GROUP_LEAVE),
            fn_set_group_special_title: get_func!(cqp, FN_NAME_SET_GROUP_SPECIAL_TITLE),
            fn_set_discuss_leave: get_func!(cqp, FN_NAME_SET_DISCUSS_LEAVE),
            fn_set_friend_add_request: get_func!(cqp, FN_NAME_SET_FRIEND_ADD_REQUEST),
            fn_set_group_add_request: get_func!(cqp, FN_NAME_SET_GROUP_ADD_REQUEST),
            fn_set_group_add_request_v2: get_func!(cqp, FN_NAME_SET_GROUP_ADD_REQUEST_V2),
            fn_get_group_member_info_v2: get_func!(cqp, FN_NAME_GET_GROUP_MEMBER_INFO_V2),
            fn_get_stranger_info: get_func!(cqp, FN_NAME_GET_STRANGER_INFO),
            fn_get_group_list: get_func!(cqp, FN_NAME_GET_GROUP_LIST),
            fn_get_group_member_list: get_func!(cqp, FN_NAME_GET_GROUP_MEMBER_LIST),
            fn_add_log: get_func!(cqp, FN_NAME_ADD_LOG),
            fn_get_cookies: get_func!(cqp, FN_NAME_GET_COOKIES),
            fn_get_csrf_token: get_func!(cqp, FN_NAME_GET_CSRF_TOKEN),
            fn_get_login_qq: get_func!(cqp, FN_NAME_GET_LOGIN_QQ),
            fn_get_login_nick: get_func!(cqp, FN_NAME_GET_LOGIN_NICK),
            fn_get_app_directory: get_func!(cqp, FN_NAME_GET_APP_DIRECTORY),
            fn_set_fatal: get_func!(cqp, FN_NAME_SET_FATAL),
            fn_get_record: get_func!(cqp, FN_NAME_GET_RECORD),
            fn_get_record_v2: get_func!(cqp, FN_NAME_GET_RECORD_V2),
            fn_get_image: get_func!(cqp, FN_NAME_GET_IMAGE),
            fn_can_send_image: get_func!(cqp, FN_NAME_CAN_SEND_IMAGE),
            fn_can_send_record: get_func!(cqp, FN_NAME_CAN_SEND_RECORD),
        } };
        return Ok(api);
    }

    pub unsafe fn send_private_msg(&self, auth_code: i32, qq: i64,
        msg: *const c_char)
    -> i32 {
        (self.fn_send_private_msg)(auth_code, qq, msg)
    }

    pub unsafe fn send_group_msg(&self, auth_code: i32, group_id: i64,
        msg: *const c_char)
    -> i32 {
        (self.fn_send_group_msg)(auth_code, group_id, msg)
    }

    pub unsafe fn send_discuss_msg(&self, auth_code: i32, discuss_id: i64,
        msg: *const c_char)
    -> i32 {
        (self.fn_send_discuss_msg)(auth_code, discuss_id, msg)
    }

    pub unsafe fn delete_msg(&self, auth_code: i32, msg_id: i64) -> i32 {
        (self.fn_delete_msg)(auth_code, msg_id)
    }

    pub unsafe fn send_like(&self, auth_code: i32, qq: i64) -> i32 {
        (self.fn_send_like)(auth_code, qq)
    }

    pub unsafe fn send_like_v2(&self, auth_code: i32, qq: i64, times: i32) -> i32 {
        (self.fn_send_like_v2)(auth_code, qq, times)
    }

    pub unsafe fn set_group_kick(&self, auth_code: i32,
        group_id: i64, qq: i64, reject: CQBool)
    -> i32 {
        (self.fn_set_group_kick)(auth_code, group_id, qq, reject)
    }

    pub unsafe fn set_group_ban(&self, auth_code: i32,
        group_id: i64, qq: i64, duration: i64)
    -> i32 {
        (self.fn_set_group_ban)(auth_code, group_id, qq, duration)
    }

    pub unsafe fn set_group_admin(&self, auth_code: i32,
        group_id: i64, qq: i64, set: CQBool)
    -> i32 {
        (self.fn_set_group_admin)(auth_code, group_id, qq, set)
    }

    pub unsafe fn set_group_whole_ban(&self, auth_code: i32,
        group_id: i64, enabled: CQBool)
    -> i32 {
        (self.fn_set_group_whole_ban)(auth_code, group_id, enabled)
    }

    pub unsafe fn set_group_anonymous_ban(&self, auth_code: i32,
        group_id: i64, anomymous: *const c_char, duration: i64)
    -> i32 {
        (self.fn_set_group_anonymous_ban)(auth_code,
            group_id, anomymous, duration)
    }

    pub unsafe fn set_group_anonymous(&self, auth_code: i32, group_id: i64,
        enabled: CQBool)
    -> i32 {
        (self.fn_set_group_anonymous)(auth_code, group_id, enabled)
    }

    pub unsafe fn set_group_card(&self, auth_code: i32,
        group_id: i64, qq: i64, card: *const c_char)
    -> i32 {
        (self.fn_set_group_card)(auth_code, group_id, qq, card)
    }

    pub unsafe fn set_group_leave(&self, auth_code: i32, group_id: i64,
        is_dismiss: CQBool)
    -> i32 {
        (self.fn_set_group_leave)(auth_code, group_id, is_dismiss)
    }

    pub unsafe fn set_group_special_title(&self, auth_code: i32,
        group_id: i64, qq: i64, title: *const c_char, duration: i64)
    -> i32 {
        (self.fn_set_group_special_title)(auth_code,
            group_id, qq, title, duration)
    }

    pub unsafe fn set_discuss_leave(&self, auth_code: i32, discuss_id: i64)
    -> i32 {
        (self.fn_set_discuss_leave)(auth_code, discuss_id)
    }

    pub unsafe fn set_friend_add_request(&self, auth_code: i32,
        response_flag: *const c_char, response_operation: i32,
        remark: *const c_char)
    -> i32 {
        (self.fn_set_friend_add_request)(auth_code,
            response_flag, response_operation, remark)
    }

    pub unsafe fn set_group_add_request(&self, auth_code: i32,
        response_flag: *const c_char, request_type: i32,
        response_operation: i32)
    -> i32 {
        (self.fn_set_group_add_request)(auth_code,
            response_flag, request_type, response_operation)
    }

    pub unsafe fn set_group_add_request_v2(&self, auth_code: i32,
        response_flag: *const c_char, request_type: i32,
        response_operation: i32, reason: *const c_char)
    -> i32 {
        (self.fn_set_group_add_request_v2)(auth_code,
            response_flag, request_type, response_operation, reason)
    }

    pub unsafe fn get_group_member_info_v2(&self, auth_code: i32,
        group_id: i64, qq: i64, nocache: CQBool)
    -> *const c_char {
        (self.fn_get_group_member_info_v2)(auth_code, group_id, qq, nocache)
    }

    pub unsafe fn get_stranger_info(&self, auth_code: i32, qq: i64,
        nocache: CQBool)
    -> *const c_char {
        (self.fn_get_stranger_info)(auth_code, qq, nocache)
    }

    pub unsafe fn get_group_list(&self, auth_code: i32) -> *const c_char {
        (self.fn_get_group_list)(auth_code)
    }

    pub unsafe fn get_group_member_list(&self, auth_code: i32,
        group_id: i64)
    -> *const c_char {
        (self.fn_get_group_member_list)(auth_code, group_id)
    }

    pub unsafe fn add_log(&self, auth_code: i32, level: i32,
        category: *const c_char, content: *const c_char)
    -> i32 {
        (self.fn_add_log)(auth_code, level, category, content)
    }

    pub unsafe fn get_cookies(&self, auth_code: i32) -> *const c_char {
        (self.fn_get_cookies)(auth_code)
    }

    pub unsafe fn get_csrf_token(&self, auth_code: i32) -> i32 {
        (self.fn_get_csrf_token)(auth_code)
    }

    pub unsafe fn get_login_qq(&self, auth_code: i32) -> i64 {
        (self.fn_get_login_qq)(auth_code)
    }

    pub unsafe fn get_login_nick(&self, auth_code: i32) -> *const c_char {
        (self.fn_get_login_nick)(auth_code)
    }

    pub unsafe fn get_app_directory(&self, auth_code: i32) -> *const c_char {
        (self.fn_get_app_directory)(auth_code)
    }

    pub unsafe fn set_fatal(&self, auth_code: i32, content: *const c_char)
    -> i32 {
        (self.fn_set_fatal)(auth_code, content)
    }

    pub unsafe fn get_record(&self, auth_code: i32, filename: *const c_char,
        format: *const c_char)
    -> *const c_char {
        (self.fn_get_record)(auth_code, filename, format)
    }

    pub unsafe fn get_record_v2(&self, auth_code: i32, filename: *const c_char,
        format: *const c_char)
    -> *const c_char {
        (self.fn_get_record_v2)(auth_code, filename, format)
    }

    pub unsafe fn get_image(&self, auth_code: i32, filename: *const c_char)
    -> *const c_char {
        (self.fn_get_image)(auth_code, filename)
    }

    pub unsafe fn can_send_image(&self, auth_code: i32) -> CQBool {
        (self.fn_can_send_image)(auth_code)
    }

    pub unsafe fn can_send_record(&self, auth_code: i32) -> CQBool {
        (self.fn_can_send_record)(auth_code)
    }
}
