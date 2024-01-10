use zenoh_msg_test;
use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};
use async_std;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let topic = "test";

    let subscriber = session.declare_subscriber(topic).res().await.unwrap();

    loop {
        let sample = subscriber.recv().unwrap();

        let str_value = sample.value.to_string();

        let deserialized = zenoh_msg_test::deserialize(str_value);

        println!("Subscribed");
        println!("name:{}, age:{}, favorite_number:{}", deserialized.name, deserialized.age, deserialized.favorite_number);
    }
}
