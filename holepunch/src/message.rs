use serde::{Deserialize, Serialize};

pub type MessageType = i64;

pub const MESSAGE_TYPE_ACK: MessageType = 0;
pub const MESSAGE_TYPE_CONNECT: MessageType = 1;
pub const MESSAGE_TYPE_PUNCHING_NEGOTIATION: MessageType = 2;
pub const MESSAGE_TYPE_START_PUNCHING: MessageType = 3;
pub const MESSAGE_TYPE_EMPTY: MessageType = 4;
pub const MESSAGE_TYPE_CONNECTION_ACK: MessageType = 5;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub identitiy_token: String,
    pub error_info: String,
    pub data: Vec<u8>,
    pub src_public_addr: String,
}

impl Message {
    pub fn new_connect_msg(data: Vec<u8>, token: String) -> Self {
        Message {
            message_type: MESSAGE_TYPE_CONNECT,
            identitiy_token: token,
            error_info: "".to_string(),
            data,
            src_public_addr: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type() {
        for i in 0..6 {
            let message = Message {
                message_type: i,
                identitiy_token: "test".to_string(),
                error_info: "test".to_string(),
                data: vec![1, 2, 3],
                src_public_addr: "".to_string(),
            };

            let json = serde_json::to_string(&message).unwrap();
            let test_message: Message = serde_json::from_str(&json).unwrap();
            assert_eq!(test_message.message_type, i);

            // switch i
            match i {
                0 => assert_eq!(test_message.message_type, MESSAGE_TYPE_ACK),
                1 => assert_eq!(test_message.message_type, MESSAGE_TYPE_CONNECT),
                2 => assert_eq!(test_message.message_type, MESSAGE_TYPE_PUNCHING_NEGOTIATION),
                3 => assert_eq!(test_message.message_type, MESSAGE_TYPE_START_PUNCHING),
                4 => assert_eq!(test_message.message_type, MESSAGE_TYPE_EMPTY),
                5 => assert_eq!(test_message.message_type, MESSAGE_TYPE_CONNECTION_ACK),
                _ => panic!("Invalid message type"),
            }
        }
    }
}
