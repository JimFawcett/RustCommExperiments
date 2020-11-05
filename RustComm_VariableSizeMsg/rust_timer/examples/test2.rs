// test2.rs

/*-----------------------------------------------
  Demonstrations of StopWatch, Timer, ...
*/
use rust_timer::{StopWatch, stop_watch, timer, date_time_stamp, date_stamp, time_stamp};

fn do_work(max:usize) -> u128 {
  let mut sw = StopWatch::new();
  sw.start();
  let mut val:f64 = 3.1415927;
  let delta:f64 = 0.05;
  for _i in 0..max {
    val = 1.0 + val/(1.0 + delta);
  }
  let _ = sw.stop();
  print!("\n  final value = {:.6}", val);
  sw.elapsed_micros()
}
fn main() {
    print!("\n  === demo date_time_timer ===");
    println!();

    print!("\n  -- doWork --");
    let max = 1000;
    print!("\n  elapsed time: {} microsec for {} iterations", do_work(max), max);
    println!();
    
    print!("\n  -- demo StopWatch --");
    stop_watch(25);
    println!();
    
    print!("\n  -- demo Timer --");
    print!("\n  starting timer(200)");
    let handle = timer(200);
    print!("\n  do some work while waiting for timer");
    let _ = handle.join(); 
    println!();
    
    print!("\n  -- demo DateTimeStamp --");
    print!("\n  now is:  {:?}", date_time_stamp());
    print!("\n  date is: {:?}", date_stamp());
    print!("\n  time is: {:?}", time_stamp());
    println!("\n\n  That's all Folks!\n\n");
}
