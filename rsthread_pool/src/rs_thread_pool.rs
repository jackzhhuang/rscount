use std::{error::Error, sync::{mpsc::{Sender, Receiver}, Arc, Mutex}};


pub struct PoolMessage<MessageType> {
    stop: bool,
    message: Option<MessageType>,
}

pub trait RsReceiver<MessageType> {
   fn process_message(&mut self, message: MessageType) -> Result<(), Box<dyn Error>>;
}

pub struct RsThreadPool<MessageType: Send + 'static> {
    count_parallelism: usize,
    sender: Option<Sender<PoolMessage<MessageType>>>,
    receiver: Option<Arc<Mutex<Receiver<PoolMessage<MessageType>>>>>,
    handles: Vec<std::thread::JoinHandle<()>>,
}

impl<MessageType: Send + 'static> RsThreadPool<MessageType> {
    pub fn new() -> RsThreadPool::<MessageType> {
        RsThreadPool::<MessageType> { 
            count_parallelism: 0, 
            sender: None, 
            receiver: None, 
            handles: vec![],
        }
    }
    fn determine_count_parallelism(&mut self) -> Result<(), Box<dyn Error>>  {
        self.count_parallelism = std::thread::available_parallelism()?.get();
        Ok(())
    }

    pub fn send(&self, m: MessageType) -> Result<(), Box<dyn Error>> {
        self.sender.as_ref().unwrap().send(PoolMessage { stop: false, message: Some(m)  })?;
        Ok(())
    }

    pub fn join(self) -> Result<(), Box<dyn Error>> {
        for _i in 0..self.handles.len() {
            self.sender.as_ref().unwrap().send(PoolMessage { stop: true, message: None  })?;
        }
        self.handles.into_iter().for_each(|h| {
            h.join().unwrap();
        }); 
        Ok(())
    }

    pub fn set_up_pool<F, ProcessorType>(&mut self, mut processor_maker: F) -> Result<(), Box<dyn Error>> 
            where F: FnMut() -> ProcessorType,
                  F: 'static,
                  ProcessorType: RsReceiver<MessageType> + Send + 'static
                  {
        self.determine_count_parallelism()?;

        let (sender, receiver) = std::sync::mpsc::channel::<PoolMessage<MessageType>>();

        self.sender = Some(sender);
        self.receiver = Some(Arc::new(Mutex::new(receiver)));

        for _i in 0..self.count_parallelism {
            let thread_receiver = Arc::clone(self.receiver.as_ref().unwrap());

            let mut processor = processor_maker();
            let handle = std::thread::spawn(move || {
                loop {
                    let receiver_result = thread_receiver.try_lock();
                    match receiver_result {
                        Ok(receiver) => {
                            let message_result = receiver.try_recv();
                            match message_result {
                                Ok(message) => {
                                    if message.stop {
                                        return;
                                    }
                                    processor.process_message(message.message.unwrap()).unwrap();
                                },
                                Err(e) => {
                                    // dbg!("error: {}", e.to_string());
                                },
                            }
                        },
                        Err(e) => {
                            // dbg!("error: {}", e.to_string());
                        },
                    }
                    
                }
           });

           self.handles.push(handle);
        }

        Ok(())
    }
}