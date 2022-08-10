use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {

    pub text: String

}