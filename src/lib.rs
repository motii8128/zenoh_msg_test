use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Message
{
    pub name:String,
    pub age:i32,
    pub favorite_number:f32
}

pub fn serialize(msg:Message)->String
{
    serde_json::to_string(&msg).unwrap()
}

pub fn deserialize(msg:String)->Message
{
    let result:Message = serde_json::from_str(&msg).unwrap();

    result
}