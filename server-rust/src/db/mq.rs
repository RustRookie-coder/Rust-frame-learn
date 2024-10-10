use std::process;

use amqp::{Basic, Channel, ConsumeBuilder, Session, Table};

pub async fn init_message_queue() -> Channel {
    let mut session = match Session::open_url("amqp://test:bix-test@127.0.0.1/test") {
        Ok(session) => { session }
        Err(error) => panic!("Can't create session: {:?}", error)
    };

    //Create a sender
    // 打开一个通道
    let mut channel = match session.open_channel(1) {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Failed to create channel: {}", e);
            process::exit(1);
        }
    };

    let exchange_type = "direct";

    let exchanges = vec!["rust.exchange", "group.exchange", "refresh.exchange"];

    for name in exchanges {
        let exchange_declare = channel.queue_declare(name, false, true, false, false, false, Table::new());
        println!("Exchange declare name : {:?} is : {:?}", name, exchange_declare.ok().is_some())
    }

    // channel.basic_publish("", "my_queue_name", true, false,
    //                       amqp::protocol::basic::BasicProperties {
    //                           content_type:
    //                           Some("text".to_string()),
    //                           ..Default::default()
    //                       }, (b"Hello from rust!").to_vec()).expect("Send message failed");
    // send_messages(channel);
    return channel;
}

fn send_messages(exchange_name: String, mut channel: Channel) {
    // 发送一条消息到队列
    let result = channel.basic_publish("rust.exchange", "rust", false, false,
                                       amqp::protocol::basic::BasicProperties::default(), (b"Hello from rust!").to_vec());
    match result {
        Ok(()) => {
            println!("Message sent");
        }
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            process::exit(1);
        }
    }
}

fn consume_messages(mut channel: Channel) {
    // let consume_builder = ConsumeBuilder::new(consumer_function, "rust");
    // let consumer_name = consume_builder.basic_consume(&mut channel);
    // println!("Starting consumer {:?}", consumer_name);
    // channel.basic_consume()
}
