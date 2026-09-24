use std::io::{BufRead, Write};
use std::io::{BufReader, BufWriter};
use std::os::unix::net::UnixStream;
use std::str::FromStr;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use boringtun::device::{DeviceConfig, Device, DeviceHandle};
//WireGuard管理
use boringtun::noise::{Tunn, TunnResult};
use tokio::net::UdpSocket;
use crate::config::AppConfig;
use crate::core::tools::{load_private_key, load_public_key, load_string_byte_32, load_string_byte_32_not_unwrap};
pub fn new_tunn(app_config: AppConfig) -> Tunn {
    Tunn::new(load_private_key(&app_config.wg_private_key).unwrap(),
              load_public_key(&app_config.wg_public_key).unwrap(),
              None, None, 0, None)
}


const WG_SOCKET: &str = "/var/run/wireguard/utun7.sock";


fn configure_peer(
    app_config: &AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {

    let private_key = load_private_key(
        &app_config.wg_private_key
    )?;

    let peer_public_key = load_public_key(
        &app_config.wg_public_key
    )?;

    let private_key_hex = hex::encode(
        private_key.as_bytes()
    );

    let peer_public_key_hex = hex::encode(
        peer_public_key.as_bytes()
    );

    println!("connect to {}", WG_SOCKET);

    let stream = UnixStream::connect(WG_SOCKET)?;

    let mut reader = BufReader::new(
        stream.try_clone()?
    );

    let mut writer = BufWriter::new(stream);

    writeln!(writer, "set=1")?;

    writeln!(
        writer,
        "private_key={}",
        private_key_hex
    )?;

    writeln!(
        writer,
        "listen_port=0"
    )?;

    writeln!(
        writer,
        "public_key={}",
        peer_public_key_hex
    )?;

    writeln!(
        writer,
        "endpoint={}",
        app_config.wg_endpoint
    )?;

    writeln!(
        writer,
        "persistent_keepalive_interval=25"
    )?;

    writeln!(
        writer,
        "replace_allowed_ips=true"
    )?;

    writeln!(
        writer,
        "allowed_ip=fd20:20:20::2/128"
    )?;

    // peer commit
    writeln!(writer)?;

    writer.flush()?;

    let mut response = String::new();

    loop {
        response.clear();

        let n = reader.read_line(&mut response)?;

        if n == 0 {
            break;
        }

        print!("WG API: {}", response);

        if response.starts_with("errno=") {
            break;
        }
    }

    Ok(())
}

pub async fn new_peer(app_config: AppConfig){
    println!("1. create device");

    let mut device = DeviceHandle::new(
        app_config.iframe_name.as_str(),
        DeviceConfig::default(),
    ).unwrap();

    println!("2. device created");

    configure_peer(&app_config).unwrap();

    println!("4. WireGuard peer configured");

    device.wait()


}


pub async fn connect(app_config: AppConfig){

    let mut out = vec![0u8; 2048];
    let mut tunn = new_tunn(app_config.clone());

    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

    let server_addr = &app_config.wg_endpoint.clone();

    println!("WireGuard endpoint: {}", server_addr);

    match tunn.format_handshake_initiation(&mut out, false) {
        TunnResult::WriteToNetwork(packet) => {
            socket
                .send_to(packet, &server_addr)
                .await
                .unwrap();

            println!("Handshake sent");
        }

        result => {
            println!("Tunn result: {:?}", result);
        }
    }
}