use std::{error::Error, sync::{mpsc::{Sender, Receiver}, Arc, Mutex}};


pub struct PoolMessage<MessageType> {
    stop: bool,
    message: MessageType,
}

pub trait RsReceiver<MessageType> {
   fn process_message(&mut self, message: MessageType) -> Result<(), Box<dyn Error>>;
}

pub struct RsThreadPool<MessageType: Send + 'static> {
    count_parallelism: usize,
    sender: Sender<PoolMessage<MessageType>>,
    receiver: Arc<Mutex<Receiver<PoolMessage<MessageType>>>>,
    handles: Vec<std::thread::JoinHandle<()>>,
}

impl<MessageType: Send + 'static> RsThreadPool<MessageType> {
    fn determine_count_parallelism(&mut self) -> Result<(), Box<dyn Error>>  {
        self.count_parallelism = std::thread::available_parallelism()?.get();
        Ok(())
    }

    fn set_up_pool<F, ProcessorType>(&mut self, mut processor_maker: F) -> Result<(), Box<dyn Error>> 
            where F: Fn() -> ProcessorType,
                  F: 'static,
                  ProcessorType: RsReceiver<MessageType> + Send + 'static
                  {
        self.determine_count_parallelism()?;

        let (sender, receiver) = std::sync::mpsc::channel::<PoolMessage<MessageType>>();

        self.sender = sender;
        self.receiver = Arc::new(Mutex::new(receiver));

        for i in 0..self.count_parallelism {
            let thread_receiver = Arc::clone(&self.receiver);

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
                                    processor.process_message(message.message).unwrap();
                                },
                                _ => {

                                },
                            }
                        },
                        _ => {

                        },
                    }
                    
                }
           });

           self.handles.push(handle);
        }

        Ok(())
    }
}