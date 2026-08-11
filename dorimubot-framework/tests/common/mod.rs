use dorimubot_framework_core::{CredentialConfig, QQBotConfig};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::OnceLock;
use std::thread;

static MOCK_API_BASE_URL: OnceLock<String> = OnceLock::new();

pub fn qqbot_config() -> QQBotConfig {
    let base_url = MOCK_API_BASE_URL.get_or_init(|| {
        let base_url = start_mock_server();
        std::env::set_var("QQ_TOKEN_URL", format!("{base_url}/token"));
        base_url
    });

    QQBotConfig::new()
        .credential(CredentialConfig {
            app_id: "test-app-id".to_string(),
            secret: "test-secret".to_string(),
        })
        .api_override(base_url)
}

fn start_mock_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock QQ API");
    let address = listener.local_addr().expect("read mock QQ API address");

    thread::Builder::new()
        .name("mock-qq-api".to_string())
        .spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.expect("accept mock QQ API connection");
                handle_request(stream);
            }
        })
        .expect("start mock QQ API");

    format!("http://{address}")
}

fn handle_request(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone mock QQ API stream"));
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .expect("read mock QQ API request line");

    let mut content_length = 0;
    loop {
        let mut header = String::new();
        reader
            .read_line(&mut header)
            .expect("read mock QQ API header");
        if header == "\r\n" || header.is_empty() {
            break;
        }

        if let Some(value) = header.to_ascii_lowercase().strip_prefix("content-length:") {
            content_length = value.trim().parse().expect("parse content length");
        }
    }

    let mut body = vec![0; content_length];
    reader
        .read_exact(&mut body)
        .expect("read mock QQ API request body");

    let path = request_line.split_whitespace().nth(1).unwrap_or_default();
    let (status, response_body) = match path {
        "/token" => (
            "200 OK",
            r#"{"access_token":"test-token","expires_in":7200}"#,
        ),
        "/users/@me" => (
            "200 OK",
            r#"{"id":"test-bot-id","username":"test-bot","bot":true,"union_openid":"test-bot-union-openid"}"#,
        ),
        _ => ("404 Not Found", r#"{"message":"not found"}"#),
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
        response_body.len()
    );

    stream
        .write_all(response.as_bytes())
        .expect("write mock QQ API response");
}
