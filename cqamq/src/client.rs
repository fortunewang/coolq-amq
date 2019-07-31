use std::sync::Arc;
use failure::{Fallible, err_msg};
use lapin::options;
use lapin::types::FieldTable;
use lapin::message::Delivery;
use lapin::{Connection, ConnectionProperties, Channel, ConsumerDelegate};
use cqrs::{gb18030, gb18030_decode};

struct RPCSubscriber {
    auth_code: i32,
    cqp: Arc<cqrs::API>,
    channel: Channel,
}

impl RPCSubscriber {

    pub fn dispatch(&self, delivery: &Delivery) -> Fallible<Vec<u8>> {
        let payload = serde_json::from_slice::<serde_json::Value>(&delivery.data)?;
        let payload = payload.as_object()
            .ok_or(err_msg("payload is not a object"))?;
        let api = payload.get("api")
            .ok_or(err_msg("payload.api is required"))?
            .as_str()
            .ok_or(err_msg("payload.api is not a string"))?;
        let params = payload.get("params");
        let res = match api {
            "send_private_message" => {
                self.send_private_message(
                    params.ok_or(err_msg("payload.params is required by send_private_message"))?
                )?
            },
            "send_group_message" => {
                self.send_group_message(
                    params.ok_or(err_msg("payload.params is required by send_group_message"))?
                )?
            },
            "send_discuss_message" => {
                self.send_discuss_message(
                    params.ok_or(err_msg("payload.params is required by send_discuss_message"))?
                )?
            },
            _ => return Err(err_msg("invalid api")),
        };
        Ok(res)
    }

    fn send_private_message(&self, params: &serde_json::Value) -> Fallible<Vec<u8>> {
        let qq = params.get("to")
            .ok_or(err_msg("payload.params.to is required by send_private_message"))?
            .as_i64()
            .ok_or(err_msg("payload.params.to is not an integer"))?;
        let message = params.get("message")
            .ok_or(err_msg("payload.params.message is required by send_private_message"))?
            .as_str()
            .ok_or(err_msg("payload.params.message is not a string"))?;
        unsafe {
            self.cqp.send_private_msg(self.auth_code, qq, gb18030!(message));
        }
        let res = crate::message::SendPrivateMessageResponse { ok: true };
        Ok(serde_json::to_vec(&res)?)
    }

    fn send_group_message(&self, params: &serde_json::Value) -> Fallible<Vec<u8>> {
        let group = params.get("group")
            .ok_or(err_msg("payload.params.group is required by send_group_message"))?
            .as_i64()
            .ok_or(err_msg("payload.params.group is not an integer"))?;
        let message = params.get("message")
            .ok_or(err_msg("payload.params.message is required by send_group_message"))?
            .as_str()
            .ok_or(err_msg("payload.params.message is not a string"))?;
        unsafe {
            self.cqp.send_group_msg(self.auth_code, group, gb18030!(message));
        }
        let res = crate::message::SendGroupMessageResponse { ok: true };
        Ok(serde_json::to_vec(&res)?)
    }

    fn send_discuss_message(&self, params: &serde_json::Value) -> Fallible<Vec<u8>> {
        let discuss = params.get("discuss")
            .ok_or(err_msg("payload.params.discuss is required by send_discuss_message"))?
            .as_i64()
            .ok_or(err_msg("payload.params.discuss is not an integer"))?;
        let message = params.get("message")
            .ok_or(err_msg("payload.params.message is required by send_discuss_message"))?
            .as_str()
            .ok_or(err_msg("payload.params.message is not a string"))?;
        unsafe {
            self.cqp.send_discuss_msg(self.auth_code, discuss, gb18030!(message));
        }
        let res = crate::message::SendDiscussMessageResponse { ok: true };
        Ok(serde_json::to_vec(&res)?)
    }

}

impl ConsumerDelegate for RPCSubscriber {
  fn on_new_delivery(&self, delivery: Delivery) {
    match self.dispatch(&delivery) {
        Ok(res) => {
            match delivery.properties.reply_to() {
                Some(queue_name) => {
                    let props = match delivery.properties.correlation_id() {
                        Some(correlation_id) => {
                            lapin::BasicProperties::default()
                                .with_correlation_id(correlation_id.clone())
                        },
                        None => lapin::BasicProperties::default(),
                    };
                    self.channel.basic_publish(
                        "", queue_name.as_str(),
                        options::BasicPublishOptions::default(),
                        res,
                        props
                    ).as_error().expect("rpc response failed");
                },
                None => {},
            }
            self.channel.basic_ack(delivery.delivery_tag, options::BasicAckOptions::default())
                .as_error().expect("basic_ack failed");
        },
        Err(_e) => {
            self.channel.basic_reject(delivery.delivery_tag, options::BasicRejectOptions::default())
                .as_error().expect("basic_reject failed");
        },
    }
  }
}

pub struct AMQPClient {
    cqp: Arc<cqrs::API>,
    auth_code: i32,
    qq: i64,
    channel: Option<Channel>,
}

impl AMQPClient {
    pub fn new(auth_code: i32) -> Fallible<Self> {
        let cqp = Arc::new(cqrs::API::new()?);
        let qq = unsafe { cqp.get_login_qq(auth_code) };
        unsafe {
            cqp.add_log(auth_code,
                cqrs::LogLevel::Info as i32,
                gb18030!("INFO"),
                gb18030!("应用已启用，QQ：{}", qq));
        }
        Ok(Self {
            cqp, auth_code, qq,
            channel: None,
        })
    }

    fn try_connect(&mut self, config: &crate::config::Config) -> Fallible<()> {
        let conn = Connection::connect_uri(config.uri.clone(), ConnectionProperties::default()).wait()?;
        let channel = conn.create_channel().wait()?;

        channel.exchange_declare(
            "coolq.msg", "topic",
            options::ExchangeDeclareOptions::default(),
            FieldTable::default()
        ).wait()?;

        channel.basic_qos(1, options::BasicQosOptions::default()).wait()?;

        channel.exchange_declare(
            "coolq.rpc", "direct",
            options::ExchangeDeclareOptions::default(),
            FieldTable::default()
        ).wait()?;

        let queue = channel.queue_declare(
            "",
            options::QueueDeclareOptions {
                exclusive: true,
                ..options::QueueDeclareOptions::default()
            },
            FieldTable::default()
        ).wait()?;

        channel.queue_bind(
            queue.name().as_str(), "coolq.rpc", &format!("{}", self.qq),
            options::QueueBindOptions::default(),
            FieldTable::default()
        ).wait()?;

        channel.basic_consume(
            &queue, "",
            options::BasicConsumeOptions::default(),
            FieldTable::default()
        ).wait()?.set_delegate(Box::new(RPCSubscriber {
            auth_code: self.auth_code,
            cqp: Arc::clone(&self.cqp),
            channel: channel.clone(),
        }));

        self.channel = Some(channel);
        Ok(())
    }

    pub fn start(&mut self) {
        let app_dir = unsafe { gb18030_decode(self.cqp.get_app_directory(self.auth_code)).unwrap() };
        let config = match crate::config::read_config(&app_dir) {
            Ok(config) => config,
            Err(e) => {
                unsafe {
                    self.cqp.add_log(self.auth_code,
                        cqrs::LogLevel::Error as i32,
                        gb18030!("ERROR"),
                        gb18030!("读取配置文件失败: {}", e));
                }
                return;
            },
        };
        unsafe {
            self.cqp.add_log(self.auth_code,
                cqrs::LogLevel::Info as i32,
                gb18030!("INFO"),
                gb18030!("连接至服务器: {}@{}:{}, vhost = {}",
                    config.uri.authority.userinfo.username,
                    config.uri.authority.host,
                    config.uri.authority.port,
                    config.uri.vhost));
        }
        match self.try_connect(&config) {
            Ok(_) => {
                unsafe {
                    self.cqp.add_log(self.auth_code,
                        cqrs::LogLevel::Info as i32,
                        gb18030!("INFO"),
                        gb18030!("服务器连接成功"));
                }
            },
            Err(e) => {
                unsafe {
                    self.cqp.add_log(self.auth_code,
                        cqrs::LogLevel::Error as i32,
                        gb18030!("ERROR"),
                        gb18030!("服务器连接失败: {}", e));
                }
            },
        }
    }

    pub fn send_message(&self, message_type: &str, payload: Vec<u8>) {
        if self.channel.is_none() { return; }
        let channel = self.channel.as_ref().unwrap();
        let res = channel.basic_publish(
            "coolq.msg", &format!("{}.{}", self.qq, message_type),
            options::BasicPublishOptions::default(),
            payload,
            lapin::BasicProperties::default()
        ).wait();
        if let Err(e) = res {
            unsafe {
                self.cqp.add_log(self.auth_code,
                    cqrs::LogLevel::Error as i32,
                    gb18030!("ERROR"),
                    gb18030!("发送消息失败: {}", e));
            }
        }
    }
}
