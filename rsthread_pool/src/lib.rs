use std::error::Error;

pub mod rs_thread_pool;
use rs_thread_pool::RsThreadPool;
use rs_thread_pool::RsReceiver;

#[derive(Debug)]
struct Person {
    age: u32,
    name: String,
}

struct PersonProcess {
   process_num: i32, 
}

impl PersonProcess {
    fn new(num: i32) -> PersonProcess {
        PersonProcess {  
            process_num: num,
        }
    }
    
}

impl RsReceiver<Person> for PersonProcess {
    fn process_message(&mut self, message: Person) -> Result<(), Box<dyn Error>> {
        println!("process num: {}, person data = {:?}", self.process_num, message);
        Ok(())
    }
}

fn test_thread_pool() -> Result<(), Box<dyn Error>> {
    let mut pool = RsThreadPool::<Person>::new();
    let mut num = 0;
    pool.set_up_pool(move || {
        num += 1;
        PersonProcess::new(num)
    });

    pool.send(Person { age: 99, name: String::from("jack") })?;
    pool.send(Person { age: 199, name: String::from("rose") })?;
    pool.send(Person { age: 299, name: String::from("jenny") })?;
    pool.send(Person { age: 399, name: String::from("bill") })?;
    pool.send(Person { age: 499, name: String::from("linus") })?;
    pool.send(Person { age: 599, name: String::from("rust") })?;

    pool.join().unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>{
        test_thread_pool()
    }
}
