extern crate amqp;
extern crate env_logger;

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlasticHeartbeat {
}

pub fn from(data: &Vec<u8>) -> Result<PlasticHeartbeat, serde_json::error::Error> {
    return serde_json::from_slice(&data);
}
