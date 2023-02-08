use std::error::Error;
use std::sync::{Arc, Mutex};

pub mod rs_thread_pool;
use rs_thread_pool::RsThreadPool;
use rs_thread_pool::RsReceiver;
use rs_thread_pool::RsCallBack;

#[derive(Debug)]
struct Person {
    age: u32,
    name: String,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person.age = {}, Person.name = {}", self.age, self.name)
    }
}

#[derive(Debug)]
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
        println!("process num: {}, person data = {}", self.process_num, message.to_string());
        Ok(())
    }
}

struct TestCallback {

}

impl RsCallBack<PersonProcess, Person> for TestCallback {
    fn callback(&mut self, t: &PersonProcess) -> Result<(), Box<dyn Error>> {
        println!("processor: {:?} ended", t);
        Ok(())
    }
}

fn test_thread_pool() -> Result<(), Box<dyn Error>> {
    let mut pool = RsThreadPool::<Person>::new();
    let mut num = 0;
    let callback = Arc::new(Mutex::new(TestCallback{}));
    pool.set_up_pool(move || -> PersonProcess {
        num += 1;
        PersonProcess::new(num)
    }, &callback).unwrap();

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
