use std::io::{BufRead, BufReader, Write};

use lunatic::{net, Process};

#[test]
fn tcp_test() {
    let server = Process::spawn_with((), |_| {
        let listener = net::TcpListener::bind("127.0.0.1:3337").unwrap();
        let tcp_stream = listener.accept().unwrap();
        let mut buf_reader = BufReader::new(tcp_stream.clone());
        let mut buffer = String::new();
        buf_reader.read_line(&mut buffer).unwrap();
        let result = buffer.contains("test");
        assert!(result);
    });

    Process::sleep(1);

    let client = Process::spawn_with((), |_| {
        let mut tcp_stream = net::TcpStream::connect("127.0.0.1:3337").unwrap();
        tcp_stream.write("test".as_bytes()).unwrap();
    });

    assert!(server.join().is_ok());
    assert!(client.join().is_ok());
}
