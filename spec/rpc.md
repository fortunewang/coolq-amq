# 消息格式：接口调用

## send_private_message

请求：

    api : 'send_private_message'
    params : object {
        to : number
        message : string
    }

响应：

    ok : true

## send_group_message

请求：

    api : 'send_group_message'
    params : object {
        group : number
        message : string
    }

响应：

    ok : true

## send_discuss_message

请求：

    api : 'send_discuss_message'
    params : object {
        discuss : number
        message : string
    }

响应：

    ok : true
