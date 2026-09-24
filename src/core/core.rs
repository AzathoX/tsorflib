use std::collections::HashMap;
use axum::async_trait;
use axum::body::Bytes;
use h3::ext::Protocol;
use h3::quic::Connection;
use quinn::{Connecting, Endpoint};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


struct HttpAdapter;
struct WebSocketAdapter;
struct UdpAdapter;

struct QuicAdapter;


struct Message {
    payload: Bytes,
}


fn  new_connecting(
    target_ipv6:&String,
    domain_name:&String
) -> Result<Connecting> {
       Ok(Endpoint::client("[::]:0".parse()?)?
         .connect(target_ipv6.parse()?, domain_name)?)

}

async fn request() -> http::Result<http::Request<()>> {
  http::Request::builder()
        .method("GET")
        .uri("https://example.com/users")
        .header("Host", "example.com")
        .body(())
}

async fn send(conn: quinn::Connection){
    let domain = String::from("aaa");
    let server_name = String::from("example.com");
    let connect = new_connecting(&server_name,&domain).unwrap();
    let conn = connect.await.unwrap();
    let h3_conn = h3_quinn::Connection::new(conn);
    let (mut driver, mut send_request) = h3::client::new(h3_conn).await.unwrap();


    // HTTP/3 request
    let req: http::Request<()> = request().await.unwrap();

    // 发送 request
    let mut stream = send_request.send_request(req).await.unwrap();

    // GET 没有 body，所以直接结束 request
    stream.finish().await;

    // 接收 response
    let response = stream.recv_response().await;


    // 驱动 HTTP/3 connection


}

#[async_trait]
trait ProtocolAdapter {
    async fn receive(&mut self) -> Result<Message>;
    async fn send(&mut self, msg: Message) -> Result<()>;

    async fn exec(&mut self, msg: Message) -> Result<()>;
}




#[async_trait]
impl ProtocolAdapter for QuicAdapter {
    async fn receive(&mut self) -> Result<Message> {
        todo!()
    }

    async fn send(&mut self, msg: Message) -> Result<()> {
        Ok(())
    }

    async fn exec(&mut self, msg: Message) -> Result<()> {
        self.send(msg).await
    }
}


pub fn exec(adapter: &mut dyn ProtocolAdapter){
      let message = Message {
          payload: Default::default(),
      };
     adapter.exec(message);
}
