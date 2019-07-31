# 消息格式：消息回调

## private

    from : number
    message : string

## group

    group : number
    from : number
    message : string

## discuss

    discuss : number
    from : number
    message : string

## group_admin_changed

    group : number
    operand : number
    set : true | false

## group_member_increase

    group : number
    from : number
    operator : number
    invited : true | false
