use minimio::{Events, Interests, Poll, Registrator, TcpStream};
use std::{io, sync::mpsc::Sender, thread};

pub struct Reactor {
    handle: std::thread::JoinHandle<()>,
    registrator: Registrator,
}

impl Reactor {
    pub fn new(evt_sender: Sender<usize>) -> Reactor {
        let mut poll = Poll::new().unwrap();
        let registrator = poll.registrator();

        // Set up the epoll/IOCP event loop in a seperate thread
        let handle = thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            loop {
                println!("Waiting! {:?}", poll);
                match poll.poll(&mut events, Some(200)) {
                    Ok(..) => (),
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("INTERRUPTED: {}", e);
                        break
                    }
                    Err(e) => panic!("Poll error: {:?}, {}", e.kind(), e),
                };
                for event in &events {
                    let event_token = event.id();
                    evt_sender.send(event_token).expect("send event_token err.");
                }
            }
        });

        Reactor { handle, registrator }
    }

    pub fn register_stream_read_interest(&self, stream: &mut TcpStream, token: usize) {
        self.registrator.register(stream, token, Interests::READABLE).expect("registration err.");
    }

    pub fn stop_loop(&self) {
        self.registrator.close_loop().expect("close loop err.");
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        self.stop_loop();
    }
}