use zenoh_msg_test;
use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};
use async_std;

use std::time::Duration;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let topic = "test";

    let publisher = session.declare_publisher(topic).res().await.unwrap();

    let mut count = 0;

    loop {
        let  msg = zenoh_msg_test::Message{
            name:"Motii".to_string(),
            age:count,
            favorite_number:66.66,
        };
        let serialized = zenoh_msg_test::serialize(msg);
        println!("publish :{}", serialized);

        publisher.put(serialized).res().await.unwrap();

        count += 1;

        std::thread::sleep(Duration::from_millis(1000));
    }
}
