use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

pub(super) fn spawn_test_http_server(body: &'static str) -> (String, String) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let addr = listener.local_addr().unwrap();
    thread::spawn(move || {
        let started_at = std::time::Instant::now();
        while started_at.elapsed() < std::time::Duration::from_secs(2) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut request = [0_u8; 1024];
                    let _ = stream.read(&mut request);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = stream.write_all(response.as_bytes());
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(std::time::Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });
    (
        format!("http://{}:{}/plugin", addr.ip(), addr.port()),
        format!("{}:{}", addr.ip(), addr.port()),
    )
}
