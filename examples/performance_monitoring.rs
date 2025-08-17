//! Example: Performance monitoring and metrics logging
//! Run with: $env:RUST_LOG='info'; cargo run --example performance_monitoring

use custom_tracing_logger::{self, structured};
use tracing::{info, warn, error, instrument};
use std::time::{Instant, Duration};

struct PerformanceMetrics {
    operation: String,
    duration: Duration,
    memory_used: u64,
    cpu_usage: f64,
}

impl PerformanceMetrics {
    fn log(&self) {
        info!(
            operation = %self.operation,
            duration_ms = self.duration.as_millis() as u64,
            memory_mb = self.memory_used / 1024 / 1024,
            cpu_percent = self.cpu_usage,
            "Performance metrics"
        );
    }
}

#[instrument]
async fn expensive_computation(size: usize) -> Result<Vec<u64>, String> {
    let start = Instant::now();
    
    info!(size = size, "Starting expensive computation");
    
    // Simulate heavy computation
    let mut result = Vec::with_capacity(size);
    for i in 0..size {
        result.push((i as u64).pow(2));
        
        // Simulate some work
        if i % 10000 == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    }
    
    let duration = start.elapsed();
    
    if duration > Duration::from_millis(1000) {
        warn!(
            size = size,
            duration_ms = duration.as_millis() as u64,
            "Computation took longer than expected"
        );
    }
    
    // Log performance metrics
    let metrics = PerformanceMetrics {
        operation: "expensive_computation".to_string(),
        duration,
        memory_used: (size * 8) as u64, // Rough estimate
        cpu_usage: 85.5, // Simulated
    };
    metrics.log();
    
    structured::database_op("INSERT", "results", duration.as_millis() as u64, Some(size as u64));
    
    Ok(result)
}

#[instrument]
async fn batch_processor(batch_size: usize, num_batches: usize) {
    info!(
        batch_size = batch_size,
        num_batches = num_batches,
        total_items = batch_size * num_batches,
        "Starting batch processing"
    );
    
    let overall_start = Instant::now();
    let mut successful_batches = 0;
    let mut failed_batches = 0;
    
    for batch_id in 0..num_batches {
        let batch_start = Instant::now();
        
        match expensive_computation(batch_size).await {
            Ok(_) => {
                successful_batches += 1;
                info!(
                    batch_id = batch_id,
                    batch_duration_ms = batch_start.elapsed().as_millis() as u64,
                    "Batch completed successfully"
                );
            }
            Err(e) => {
                failed_batches += 1;
                error!(
                    batch_id = batch_id,
                    error = %e,
                    batch_duration_ms = batch_start.elapsed().as_millis() as u64,
                    "Batch processing failed"
                );
            }
        }
    }
    
    let total_duration = overall_start.elapsed();
    
    info!(
        total_duration_ms = total_duration.as_millis() as u64,
        successful_batches = successful_batches,
        failed_batches = failed_batches,
        success_rate = (successful_batches as f64 / num_batches as f64) * 100.0,
        avg_batch_time_ms = total_duration.as_millis() as u64 / num_batches as u64,
        "Batch processing completed"
    );
}

#[tokio::main]
async fn main() {
    custom_tracing_logger::init();
    
    info!("Performance monitoring example starting");
    custom_tracing_logger::print_config();
    
    // Run different workloads
    batch_processor(1000, 3).await;
    batch_processor(5000, 2).await;
    
    // Simulate some errors
    info!("Simulating error conditions");
    
    error!(
        error_code = "MEMORY_LIMIT_EXCEEDED",
        requested_mb = 1024,
        available_mb = 512,
        "Memory allocation failed"
    );
    
    warn!(
        threshold_ms = 500,
        actual_ms = 750,
        operation = "database_query",
        "Operation exceeded performance threshold"
    );
    
    info!("Performance monitoring example completed");
}