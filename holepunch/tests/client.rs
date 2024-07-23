#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use holepunch::client::TraversalTool;
    use holepunch::nat_tpye::*;

    #[test]
    fn test_new_traversal_tool() {
        let server_addr = "127.0.0.1:8080".to_string();
        let nat_info = NATTypeInfo::new(NAT_TYPE_UNKNOWN);
        let traversal_tool = TraversalTool::new(server_addr.clone(), nat_info);

        assert_eq!(traversal_tool.server_addr, server_addr);
        assert_eq!(traversal_tool.nat_info.nat_type, NAT_TYPE_UNKNOWN);
        assert_eq!(traversal_tool.local_addr, "".to_string());
    }

    #[test]
    fn test_set_local_addr() {
        let mut traversal_tool = TraversalTool::new("127.0.0.1:8080".to_string(), NATTypeInfo::new(NAT_TYPE_UNKNOWN));
        traversal_tool.set_local_addr("192.168.1.1:8080".to_string());

        assert_eq!(traversal_tool.local_addr, "192.168.1.1:8080");
    }

    #[test]
    fn test_traversal_tool_connect() {
        let server_addr = "".to_string();
        let nat_info = NATTypeInfo::new(NAT_TYPE_SYMMETRIC);

        let traversal_tool = TraversalTool::new(server_addr.clone(), nat_info);

        let mut stream = TcpStream::connect(&traversal_tool.server_addr).unwrap();

        assert!(stream.set_read_timeout(Some(std::time::Duration::from_secs(5))).is_ok());

        assert!(stream.set_write_timeout(Some(std::time::Duration::from_secs(5))).is_ok());

        assert!(traversal_tool.handle_connect(&mut stream, "test11".to_string()).is_ok());

    }
}