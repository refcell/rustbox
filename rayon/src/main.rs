use rayon::prelude::*;
use tracing::{info, Level, Instrument};
use tokio::sync::oneshot;
use tracing_subscriber::fmt::format::FmtSpan;

async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (send, recv) = oneshot::channel();

    // Spawn a task on rayon.
    rayon::spawn(move || {
        // Perform an expensive computation.
        let mut sum = 0;
        for num in nums {
            sum += num;
        }

        // Send the result back to Tokio.
        let _ = send.send(sum);
    });

    // Wait for the rayon task.
    recv.await.expect("Panic in rayon::spawn")
}

async fn mthreads_parallel_sum(nums: Vec<i32>) -> i32 {
    let (send, recv) = oneshot::channel();

    // Spawn a task on rayon.
    rayon::spawn(move || {
        // Uses the rayon parallel iterator to perform expensive operation.
        let sum = nums.par_iter().sum();

        // Send the result back to Tokio.
        let _ = send.send(sum);
    });

    // Wait for the rayon task.
    recv.await.expect("Panic in rayon::spawn")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than DEBUG (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // Events will be synthesized whenever a span is created, entered, exited, or closed.
        // The close event will contain the span's busy and idle time if timestamps are enabled.
        .with_span_events(FmtSpan::FULL)
        // sets this to be the default, global subscriber for this application.
        .init();

    let nums = vec![1; 1024 * 1024];
    info!("Parallel Summation: {}", parallel_sum(nums).instrument(tracing::info_span!("single_thread")).await);

    let mthreads_sum = vec![1; 1024 * 1024];
    info!("M Threads Parallel Summation: {}", mthreads_parallel_sum(mthreads_sum).instrument(tracing::info_span!("m_threads")).await);
}

