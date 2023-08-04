use std::{thread::{JoinHandle, self}, sync::mpsc, time::Instant, io, env};
use num_cpus;
use thousands::Seperable;

pub mod collatz;
pub mod macros;

//const COUNT: usize = millions!(100000);
//const COUNT: usize = 10000000;



fn main() {
  let args: Vec<String> = env::args().collect();
  let query = &args[1];
  let COUNT: u64 = query.parse().unwrap();

  let mut ranges: Vec<(usize, usize)> = vec![];
  let cores: usize = num_cpus::get();
  //let cores = 1;
  let step_size = (COUNT as f32) / (cores as f32);
  println!(
    ">>> Using {} threads to compute {} numbers ({:.3} per thread)",
    cores, COUNT.seperate_with_commas(), step_size);

  for step in 0..cores {
    let lo = (step_size as f32 * step as f32).ceil();
    let hi = (step_size as f32 * (step + 1) as f32).floor();
    ranges.push((lo as usize, hi as usize));
  }

  type Result = (u64, u64);
  let mut handles: Vec<JoinHandle<_>> = vec![];
  let mut results: Vec<Result> = vec![];

  let (tx, rx) = mpsc::channel();
  for (min, max) in ranges {
    let sender = tx.clone();
    handles.push(thread::spawn(move || {
      let mut largest: u64 = 0;
      let mut which: u64 = min as u64;

      for i in min..max {
        let result = collatz::compute(i as u64);
        if result > largest {
          largest = result;
          which = i as u64;
        }
      }

      sender.send((largest, which)).ok();
    }));

    println!(
      ">>> Started thread #{} for [{}, {}]",
      handles.len(), min, max);
  }

  drop(tx);

  let start = Instant::now();
  while let Ok(thread_result) = rx.recv() {
    println!(
      ">>> A thread finished: ({}, {})",
      thread_result.0, thread_result.1);

    results.push(thread_result);
  }
  let elapsed = start.elapsed().as_secs_f32();

  for handle in handles {
    let _ = handle.join();
  }

  let mut which: u64 = 0;
  let mut largest: u64 = 0;
  for (res_largest, res_which) in results {
    if res_largest > largest {
      largest = res_largest;
      which = res_which;
    }
  }

  println!(">>> Found largest: {} (#{})", collatz::compute_full(which), which);
  println!(">>> Took {:.3} s to compute", elapsed);

  //println!("Press enter to continue...");
  //io::stdin().read_line(&mut String::new()).unwrap();
}
