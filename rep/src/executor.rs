use super::TEST_TOKEN;
use std::sync::mpsc::Receiver;

pub struct Excutor {
    events: Vec<(usize, Box<dyn FnMut()>)>,
    evt_receiver: Receiver<usize>,
}

impl Excutor {
    pub fn new(evt_receiver: Receiver<usize>) -> Self {
        Excutor { events: vec![], evt_receiver }
    }

    pub fn suspend(&mut self, id: usize, f: impl FnMut() + 'static) {
        self.events.push((id, Box::new(f)));
    }

    pub fn resume(&mut self, event: usize) {
        let (_, f) =
            self.events.iter_mut().find(|(e, _)| *e == event).expect("Couldn't find event.");
        f();
    }

    pub fn block_on_all(&mut self) {
        while let Ok(received_token) = self.evt_receiver.recv() {
            assert_eq!(TEST_TOKEN, received_token, "Non matching tokens.");
            self.resume(received_token);
        }
    }
}
