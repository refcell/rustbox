use minimio::TcpStream;
use std::{
    io::{Read, Write},
    sync::mpsc::channel,
};

use rep::{executor::*, reactor::*, TEST_TOKEN};

fn main() {
    let (evt_sender, evt_receiver) = channel();
    let reactor = Reactor::new(evt_sender);
    let mut executor = Excutor::new(evt_receiver);

    let mut stream = TcpStream::connect("slowwly.robertomurray.co.uk:80").unwrap();
    let request = b"GET /delay/1000/url/http://www.google.com HTTP/1.1\r\nHost: slowwly.robertomurray.co.uk\r\nConnection: close\r\n\r\n";

    stream.write_all(request).expect("Stream write err.");
    reactor.register_stream_read_interest(&mut stream, TEST_TOKEN);

    executor.suspend(TEST_TOKEN, move || {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        println!("{}", buffer);
        assert!(!buffer.is_empty(), "Got an empty buffer");
        reactor.stop_loop();
    });

    executor.block_on_all();
    // NB! Best practice is to make sure to join our child thread. We skip it here for brevity.
    println!("EXITING");
}
