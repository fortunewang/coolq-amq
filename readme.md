# coolq-amq

AMQP adapter for [CoolQ](https://cqp.cc/);

Install cross-compiler:

    rustup target add i686-pc-windows-msvc

Build:

    cargo build --release

Then package `cqamq/app.jsonc` and `target/i686-pc-windows-msvc/cqamq.dll` according to [the tutorial](https://d.cqp.me/Pro/%E5%BC%80%E5%8F%91/%E5%BF%AB%E9%80%9F%E5%85%A5%E9%97%A8#%E6%89%93%E5%8C%85%E5%BA%94%E7%94%A8)

## 动机

目前，[酷Q](https://cqp.cc/)应用有以下限制：

* 只能运行在32位Windows ABI下
* 只能以DLL的方式导出消息回调

这对一些编程语言或库的用户来说是一个障碍。例如JavaScript用户不得不想办法在应用中嵌入一个解释器；TensorFlow用户由于官方不支持32位Windows，不得不寻找一个非官方的修改版。

因此，适宜将酷Q与其应用解耦，为应用开发争取更大的灵活性。例如酷Q的HTTP adapter： [richardchien/coolq-http-api](https://github.com/richardchien/coolq-http-api/)。

## 此应用

此应用是酷Q的AMQP adapter。

可以在应用数据目录下创建`config.toml`配置服务器相关参数：

    host = "127.0.0.1"
    port = 5672
    username = "guest"
    password = "guest"
    vhost = "/"

以上参数都是可选的，其默认值如例子所示。如果配置文件不存在，所有参数都使用默认值。

此应用在启用后会连接AMQP服务器，并声明以下交换器：

* `coolq.msg`：用于转发消息
* `coolq.rpc`：用于接收接口调用请求

所有AMQP消息体都以JSON编码。下游应用可以以任何支持AMQP的语言开发。

## 下游应用：消息回调

若要接收酷Q消息，下游应用应当在连接服务器后声明`coolq.msg`交换器，绑定队列到此交换器，并接收队列上的消息。
`coolq.msg`交换器为topic交换器，其消息路由键形如`{QQ}.{type}`。
例如，酷Q登录QQ账号1234，收到的私聊消息就会以`1234.private`为路由键发送到`coolq.msg`交换器。

据此，下游应用在绑定队列时可以有多种绑定键的形式：

* 绑定键为`1234.private`：只接收1234收到的私聊消息
* 绑定键为`1234.#`：只接收1234收到的消息，无视消息类型
* 绑定键为`#.private`：只接受私聊消息，无视酷Q登录的QQ
* 绑定键为`*`：无论消息类型和酷Q登录的QQ，接收一切消息

## 下游应用：调用酷Q接口

若要调用酷Q接口，下游应用应当在连接服务器后声明`coolq.rpc`交换器，并发送消息到`coolq.rpc`交换器。
`coolq.rpc`交换器为direct交换器，此应用会以酷Q登录的QQ为绑定键绑定队列到`coolq.rpc`并接收消息。
因此，下游应用在发送消息时必须指定酷Q登录的QQ为路由键。

下游应用可以将一个队列名设为消息的`reply-to`以接收接口调用的返回结果。
返回结果会通过AMQP默认交换器发送。

## Q&A

Q：可以让多个下游应用接收同一条消息吗？

A：可以。绑定各自的队列到`coolq.msg`到即可，消息会在此广播。

Q：可以让多个下游应用至多只有一个能接收到消息吗？

A：可以。让这些下游应用共用一个队列，消息会议round-robin的方式发送。

Q：多个酷Q可以共用一个服务器吗？

A：可以。因为QQ号是绑定键和路由键的一部分，用于匹配。

Q：多个酷Q可以处理同一条接口调用吗？

A：不可以。目前的设计中必须指定QQ以确定哪个酷Q会处理。

Q：下游应用未启动时，消息会在服务器保存吗？

A：不会。目前`coolq.msg`上的消息是非持久化的。
