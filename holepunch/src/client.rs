use std::io;
use std::io::{Read, Write};
use crate::nat_tpye::NATTypeInfo;
use log::{info, trace, warn};


pub struct TraversalInfo {
    pub local_nat: NATTypeInfo,
    pub remote_nat: NATTypeInfo,
    pub local_addr: String,
    pub remote_addr: String,
}


#[derive(Debug)]
pub struct TraversalTool {
    pub server_addr: String,
    pub nat_info: NATTypeInfo,
    pub local_addr: String,
}

impl TraversalTool {
    pub fn new(server_addr: String, nat_info: NATTypeInfo) -> Self {
        TraversalTool {
            server_addr,
            nat_info,
            local_addr: "".to_string(),
        }
    }

    pub fn set_local_addr(&mut self, local_addr: String) {
        self.local_addr = local_addr;
    }


    // traversal
    pub fn traversal(&self, token: String) -> io::Result<TraversalInfo> {
        // Connect to server
        let mut stream = match std::net::TcpStream::connect(&self.server_addr) {
            Ok(stream) => stream,
            Err(e) => return Err(e),
        };

        stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(std::time::Duration::from_secs(5)))?;

        let connect_msg = serde_json::to_vec(&crate::message::Message::new_connect_msg(
            serde_json::to_vec(&self.nat_info)?, token))?;

        // Send connect message
        stream.write_all(&connect_msg)?;


        // Receive ack message
    }

    // handle connect
    fn handle_connect(&self, stream: &mut std::net::TcpStream, token: String) -> io::Result<()> {
        let connect_msg = serde_json::to_vec(&crate::message::Message::new_connect_msg(
            serde_json::to_vec(&self.nat_info)?, token))?;

        // Send connect message
        match stream.write_all(&connect_msg) {
            Ok(_) => {
                trace!("Send connect message successfully");

                // Receive ack message
                let mut ack_msg = vec![0; 160];

                match stream.read(&mut ack_msg) {
                    Ok(_) => {

                    }
                    Err(e) => {
                        warn!("Failed to receive ack message: {}", e);
                        return Err(e);
                    }
                }

                Ok(())
            }
            Err(e) => {
                warn!("Failed to send connect message: {}", e);
                Err(e)
            }
        }
    }
}