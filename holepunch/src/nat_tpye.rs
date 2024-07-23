use serde::{Deserialize, Serialize};

pub type NATType = i64;

const NAT_TYPE_UNKNOWN: NATType = 0;
const NAT_TYPE_NONE: NATType = 1;
const NAT_TYPE_FULL_CONE: NATType = 2;
const NAT_TYPE_RESTRICTED_CONE: NATType = 3;
const NAT_TYPE_FULL_OR_RESTRICTED_CONE: NATType = 4;
const NAT_TYPE_PORT_RESTRICTED_CONE: NATType = 5;
const NAT_TYPE_SYMMETRIC: NATType = 6;


#[derive(Serialize, Deserialize, Debug)]
pub struct NATTypeInfo {
    pub nat_type: NATType
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_type() {
        for i in 0..7 {
            let nat_info = NATTypeInfo {
                nat_type: i,
            };

            let json = serde_json::to_string(&nat_info).unwrap();
            let test_nat_info: NATTypeInfo = serde_json::from_str(&json).unwrap();
            assert_eq!(test_nat_info.nat_type, i);

            // switch i
            match i {
                0 => assert_eq!(test_nat_info.nat_type, NAT_TYPE_UNKNOWN),
                1 => assert_eq!(test_nat_info.nat_type, NAT_TYPE_NONE),
                2 => assert_eq!(test_nat_info.nat_type, NAT_TYPE_FULL_CONE),
                3 => assert_eq!(test_nat_info.nat_type, NAT_TYPE_RESTRICTED_CONE),
                4 => assert_eq!(test_nat_info.nat_type, NAT_TYPE_FULL_OR_RESTRICTED_CONE),
                5 => assert_eq!(test_nat_info.nat_type, NAT_TYPE_PORT_RESTRICTED_CONE),
                6 => assert_eq!(test_nat_info.nat_type, NAT_TYPE_SYMMETRIC),
                _ => panic!("Invalid nat type"),
            }
        }
    }
}