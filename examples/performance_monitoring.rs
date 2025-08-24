//! Example: Performance monitoring with structured helpers
//! Run with: cargo run --example performance_monitoring

use custom_tracing_logger::structured;
use tracing::{info, warn, instrument};
use std::time::Instant;

#[instrument]
fn process_batch(batch_id: u64, items: usize) -> Result<(), &'static str> {
    let start = Instant::now();
    
    info!(batch_id = batch_id, items = items, "Processing batch");
    
    // Simulate work
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    let duration_ms = start.elapsed().as_millis() as u64;
    
    // Log database operation
    structured::database_op("INSERT", "batch_results", duration_ms, Some(items as u64));
    
    if duration_ms > 100 {
        warn!(
            batch_id = batch_id,
            duration_ms = duration_ms,
            threshold_ms = 100,
            "Batch processing slower than expected"
        );
    }
    
    Ok(())
}

#[instrument]
fn run_job(job_id: u64) {
    info!(job_id = job_id, "Starting job");
    
    for batch_id in 1..=3 {
        match process_batch(batch_id, 1000) {
            Ok(_) => info!(batch_id = batch_id, "Batch completed"),
            Err(e) => warn!(batch_id = batch_id, error = e, "Batch failed"),
        }
    }
    
    info!(job_id = job_id, "Job completed");
}

fn main() {
    custom_tracing_logger::init();
    
    info!("Performance monitoring started");
    
    // Run a job
    run_job(12345);
    
    // Log some user actions
    structured::user_action(123, "file_upload", Some("documents"));
    structured::user_action(456, "data_export", Some("reports"));
    
    // Log HTTP requests
    structured::http_request("GET", "/api/status", 200, 25);
    structured::http_request("POST", "/api/upload", 201, 1250);
    
    info!("Performance monitoring completed");
}