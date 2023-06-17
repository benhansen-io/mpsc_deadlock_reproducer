use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use thread_priority::ThreadPriority;

#[cfg(not(feature = "crossbeam"))]
use std::sync::mpsc::channel;
#[cfg(not(feature = "crossbeam"))]
use std::sync::mpsc::TryRecvError;

#[cfg(feature = "crossbeam")]
use crossbeam::channel::unbounded as channel;
#[cfg(feature = "crossbeam")]
use crossbeam::channel::TryRecvError;

fn main() {
    const PINNED_CORE: usize = 2;

    if cfg!(feature = "crossbeam") {
        println!("Using crossbeam::unbounded channels");
    } else {
        println!("Using std::sync::mpsc channels");
    }

    let (sender, receiver) = channel::<usize>();

    std::thread::Builder::new()
        .name("sending".to_owned())
        .spawn(move || {
            thread_priority::set_current_thread_priority(ThreadPriority::Min).unwrap();
            core_affinity::set_for_current(core_affinity::CoreId { id: PINNED_CORE });

            loop {
                sender.send(42).unwrap();
            }
        })
        .unwrap();

    let num_received = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    std::thread::Builder::new()
        .name("receiving".to_owned())
        .spawn({
            let num_received = num_received.clone();
            move || {
                thread_priority::set_current_thread_priority(ThreadPriority::Max).unwrap();
                core_affinity::set_for_current(core_affinity::CoreId { id: PINNED_CORE });

                loop {
                    let start = Instant::now();
                    let try_receive_result = receiver.try_recv();
                    let elapsed = start.elapsed();
                    if elapsed > Duration::from_secs(1) {
                        println!("try_recv blocked for {:.2} seconds", elapsed.as_secs_f32());
                    }
                    match try_receive_result {
                        Ok(_) => {
                            num_received.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        }
                        Err(TryRecvError::Empty) => {
                            std::thread::sleep(Duration::from_millis(200));
                        }
                        Err(TryRecvError::Disconnected) => unreachable!(),
                    }
                }
            }
        })
        .unwrap();

    loop {
        std::thread::sleep(Duration::from_millis(500));
        println!(
            "Receiving thread has received {}",
            num_received.load(std::sync::atomic::Ordering::SeqCst)
        )
    }
}
