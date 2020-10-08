use std::error::Error;
use std::fmt;
use std::sync::{
    Arc, 
    LockResult, 
    Mutex, 
    MutexGuard, 
    RwLock, 
    RwLockReadGuard, 
    RwLockWriteGuard
};
use quartz::wait_group::WaitGroup;


#[derive(Debug)]
pub struct WorkQueue<T>(Arc<Mutex<(Vec<T>, bool)>>);

#[derive(Debug)]
struct WorkQ<T>(Vec<T>, bool);

impl<T> Clone for WorkQueue<T> {
    fn clone(&self) -> WorkQueue<T> {
        WorkQueue::<T>(Arc::clone(&self.0))
    }
}

impl<T> WorkQueue<T> {
    fn new() -> WorkQueue<T> {
        WorkQueue::<T>(Arc::new(Mutex::new((Vec::<T>::new(), false))))
    }

    fn send(&mut self, value: T) -> Result<(), ()> {
        // TODO: CATCH THE ERR:
        match self.0.lock() {
            Ok(mut contents) => {
                contents.0.push(value);
                contents.1 = true;
                Ok(())
            },

            Err(_) => Err(())
        }
    }

    // TODO: RETURN ERR
    fn steal(&mut self) -> Option<T> {
        match self.0.try_lock() {
            Ok(mut contents) => {
                if contents.1 {

                    let x = contents.0.remove(0);
                    contents.1 = contents.0.is_empty();

                    Some(x)
                } else {
                    None
                }
            },

            Err(_) => None
        }
    }
}


