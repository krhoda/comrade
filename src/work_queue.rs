// FOR CUSTOM ERRs
// use std::error::Error;
// use std::fmt;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug)]
struct WorkQ<T>(Vec<T>, bool);

#[derive(Debug)]
struct WorkMachine<T> {
    work_q: Mutex<WorkQ<T>>,
    sender_q: RwLock<u64>,
}

impl<T> WorkMachine<T> {
    fn new() -> WorkMachine<T> {
        WorkMachine {
            work_q: Mutex::new(WorkQ::<T>(Vec::<T>::new(), false)),
            sender_q: RwLock::new(0),
        }
    }
}

#[derive(Debug)]
pub struct WorkQueue<T>(Arc<WorkMachine<T>>);

impl<T> Clone for WorkQueue<T> {
    fn clone(&self) -> WorkQueue<T> {
        WorkQueue::<T>(Arc::clone(&self.0))
    }
}

impl<T> WorkQueue<T> {
    pub fn new() -> WorkQueue<T> {
        WorkQueue::<T>(Arc::new(WorkMachine::<T>::new()))
    }

    pub fn send(&mut self, value: T) -> Result<(), ()> {
        // Set sender_q > 0 which will cause work thiefs
        // to ignore this queue and reduce the next lock's contention.
        {
            // TODO: CATCH ERR:
            let mut w = self.0.sender_q.write().unwrap();
            *w += 1;
        }

        // This is the only serious point of contention, but only one
        // thread should be attempting to place in a q at any given
        // runtime, should not be significant.
        // TODO: CATCH THE ERR:
        match self.0.work_q.lock() {
            Ok(mut contents) => {
                contents.0.push(value);
                contents.1 = true;

                // decrement sender_q
                {
                    let mut w = self.0.sender_q.write().unwrap();
                    *w -= 1;
                }

                Ok(())
            }
            Err(_) => {
                {
                    let mut w = self.0.sender_q.write().unwrap();
                    *w -= 1;
                }

                Err(())
            }
        }
    }

    // TODO: RETURN ERR
    pub fn steal(&mut self) -> Option<T> {
        // If the sender_q is contested, PLEASE GIVE UP
        match self.0.sender_q.try_read() {
            Ok(num) => {
                // if the sender_q is greater than 0 PLEASE GIVE UP,
                // let the sender do it's thing, catch it next spin lock.
                if *num > 0 {
                    None
                } else {
                    // if we're this far, we'd only be contesting
                    // against other thieves.
                    match self.0.work_q.try_lock() {
                        Ok(mut contents) => {
                            // if it has contents...
                            if contents.1 {
                                // take the first...
                                let x = contents.0.remove(0);
                                // set the state after mutation.
                                contents.1 = !contents.0.is_empty();

                                // return the value, no lnger contest the locks.
                                Some(x)
                            } else {
                                None
                            }
                        }

                        Err(_) => None,
                    }
                }
            }
            // TODO: REPORT ERR:
            _ => return None,
        }
    }
}
