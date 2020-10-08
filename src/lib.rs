pub mod work_queue;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_work_queue() {
        let mut p1 = work_queue::WorkQueue::<u64>::new();
        let mut p2 = work_queue::WorkQueue::<u64>::new();
        let mut p3 = work_queue::WorkQueue::<u64>::new();
        let mut q1 = p1.clone();
        let mut q2 = p2.clone();
        let mut q3 = p3.clone();

        let mut r1 = p1.clone();
        let mut r2 = p2.clone();
        let mut r3 = p3.clone();

        let h1 = thread::spawn(move || {
            let mut dead = false;
            while !dead {
                match q1.steal() {
                    Some(x) => {
                        println!("Thread 1 heard num: {} from queue 1", x);
                        if x == 0 {
                            return;
                        }
                    }
                    None => {}
                };

                match q2.steal() {
                    Some(x) => {
                        println!("Thread 1 heard num: {} from queue 2", x);
                        if x == 0 {
                            return;
                        }
                    }
                    None => {}
                };

                match q3.steal() {
                    Some(x) => {
                        println!("Thread 1 heard num: {} from queue 3", x);
                        if x == 0 {
                            return;
                        }
                    }
                    None => {}
                };
            }
        });

        let h2 = thread::spawn(move || {
            let dead = false;
            while !dead {
                match r1.steal() {
                    Some(x) => {
                        println!("Thread 2 heard num: {} from queue 1", x);
                        if x == 0 {
                            return;
                        }
                    }
                    None => {}
                };

                match r2.steal() {
                    Some(x) => {
                        println!("Thread 2 heard num: {} from queue 2", x);
                        if x == 0 {
                            return;
                        }
                    }
                    None => {}
                };

                match r3.steal() {
                    Some(x) => {
                        println!("Thread 2 heard num: {} from queue 3", x);
                        if x == 0 {
                            return;
                        }
                    }
                    None => {}
                };
            }
        });

        let mut ci = 0;
        let mut v = vec![p1, p2, p3];
        for i in 1..50 {
            if ci > 2 {
                ci = 0;
            }

            v[ci].send(i).unwrap();

            ci += 1;
        }

        v[0].send(0).unwrap();
        v[1].send(0).unwrap();
        v[2].send(0).unwrap();

        h1.join();
        h2.join();
        println!("THAT'S ALL FOLKS");
    }
}
