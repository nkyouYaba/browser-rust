extern crate alloc;
use crate::alloc::string::ToString;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;
use core::http::HttpResponse;
use noli::net::{lookup_host, SocketAddr, TcpStream};

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    // domain名からIPアドレスへの変換（正引き）
    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            // パターンマッチング lookup_hostの戻り値に対しての分岐
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                )))
            }
        };

        // 1つのドメイン名に対して複数のIPアドレスを返す場合がある
        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }

        // intoメソッドを使ってSocketAddrに変換
        let socket_addr: SocketAddr = (ips[0], port).into();

        // TCPストリームの確立
        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(e) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ));
            }
        };

        // リクエストラインの作成
        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");

        // ヘッダの追加
        request.push_str("Host: ");
        request.push_str(&host);
        request.push('\n');
        request.push_str("Accept: text/html\n");
        request.push_str("Connection: close\n");
        request.push('\n');

        // リクエストの送信
        // 何バイト送信したかを取得
        // unused variableの警告を抑制するために先頭にアンダーバー
        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to send a request to TCP stream".to_string(),
                ));
            }
        };

        // レスポンスの受信
        let mut recieved = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network(
                        "Failed to receive a request from TCP stream".to_string(),
                    ));
                }
            };

            // 読み込みが0バイトなら終了
            if bytes_read == 0 {
                break;
            }
            // 読み込んだバイト数だけVecに追加
            recieved.extend_from_slice(&buf[..bytes_read]);
        }
    }
}
