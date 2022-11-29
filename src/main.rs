use std::{error::Error, process, sync::Arc, thread, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn main() {
    if let Err(err) = progress_bar(5) {
        println!("Error executing cmds: {}", err);
        process::exit(1);
    }
}

fn progress_bar(num: u64) -> Result<Arc<MultiProgress>, Box<dyn Error>> {
    let m = Arc::new(MultiProgress::new());
    let sty = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {eta}",
    )
    .unwrap()
    .progress_chars("#>-");

    let pb = m.add(ProgressBar::new(num));
    pb.set_style(sty.clone());

    // make sure we show up at all.  otherwise no rendering
    // event.
    pb.tick();
    for _ in 0..num {
        let pb2 = m.add(ProgressBar::new(128));
        pb2.set_style(sty.clone());
        for _ in 0..128 {
            pb2.inc(1);
            thread::sleep(Duration::from_millis(5));
        }
        pb2.finish();
        pb.inc(1);
    }
    pb.finish_with_message("done");

    Ok(m)
}

fn waste_some_time() {
    thread::sleep(Duration::from_secs(3));
}
