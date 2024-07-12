use rosc::{OscMessage, OscPacket, OscType};
use std::net::{Ipv4Addr, UdpSocket};
use std::sync::OnceLock;

use crate::transform::Transform;

pub fn socket() -> &'static UdpSocket {
    static UDP_SOCKET: OnceLock<UdpSocket> = OnceLock::new();
    UDP_SOCKET.get_or_init(|| UdpSocket::bind("0.0.0.0:0").unwrap())
}

/// Broadcast the transform of a tracker to a target IP address and port.
pub fn send_tracker_transform(
    target: Ipv4Addr,
    port: u16,
    tracker_index: u32,
    transform: &Transform,
) {
    let payload = OscPacket::Message(OscMessage {
        addr: format!("/tracker/{}", tracker_index),
        args: vec![
            OscType::Float(transform.position.x),
            OscType::Float(transform.position.y),
            OscType::Float(transform.position.z),
            OscType::Float(transform.rotation.x),
            OscType::Float(transform.rotation.y),
            OscType::Float(transform.rotation.z),
            OscType::Float(transform.rotation.w),
        ],
    });
    let msg_buf = rosc::encoder::encode(&payload).unwrap();
    match socket().send_to(&msg_buf, format!("{}:{}", target.to_string(), port)) {
        Ok(_) => {}
        Err(err) => eprintln!("Failed to send OSC message: {:?}", err),
    }
}
