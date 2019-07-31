import json
import pika
from uuid import uuid4

class App:

    def __init__(self, params):
        self.connection = pika.BlockingConnection(params)
        self.channel = self.connection.channel()
        self.channel.exchange_declare('coolq.msg', 'topic')
        self.channel.exchange_declare('coolq.rpc', 'direct')
        res = self.channel.queue_declare(queue='', exclusive=True)
        self.queue_name = res.method.queue
        self.channel.queue_bind(self.queue_name, 'coolq.msg', '#.private')
        self.channel.basic_consume(
            queue=self.queue_name,
            on_message_callback=self.on_message,
            auto_ack=True)
        res = self.channel.queue_declare(queue='', exclusive=True)
        self.rpc_response_queue_name = res.method.queue
        self.channel.basic_consume(
            queue=self.rpc_response_queue_name,
            on_message_callback=self.on_rpc_response,
            auto_ack=True)

    def start(self):
        print('start_consuming')
        self.channel.start_consuming()
    
    def on_rpc_response(self, channel, delivery, props, payload):
        payload = json.loads(payload)
        print(f'rpc response: {payload}')

    def on_message(self, channel, delivery, props, payload):
        qq, message_type = delivery.routing_key.split('.')
        qq = int(qq)
        payload = json.loads(payload)
        message = payload['message']
        message_from = int(payload['from'])
        print(f'[{message_from}] {message}')
        if message_type == 'private':
            channel.basic_publish('coolq.rpc', str(qq), json.dumps({
                'api': 'send_private_message',
                'params': {
                    'to': message_from,
                    'message': message,
                },
            }), properties=pika.BasicProperties(
                reply_to=self.rpc_response_queue_name,
                correlation_id=str(uuid4()),
            ))

if __name__ == '__main__':
    params = pika.ConnectionParameters(
        host='localhost',
        port=5672,
        credentials=pika.PlainCredentials('guest', 'guest'),
        virtual_host='/',
    )
    App(params).start()
