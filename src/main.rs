mod solution;
mod unit_tests;
use memory_stats::memory_stats;
use solution::Solution as sol;
use std::time::Instant;

fn main() {
    println!("=== Benchmarking ===");

    // --- Small Input Test ---
    println!("[Small Input Test]");
    let s = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    s.len();
    let usage_before = memory_stats().unwrap();
    let start_time = Instant::now();

    let _ = sol::spiral_order(s.clone());

    let duration = start_time.elapsed();
    let usage_after = memory_stats().unwrap();

    let mem_used = usage_after
        .physical_mem
        .saturating_sub(usage_before.physical_mem);

    println!("Execution Time: {:?}", duration);
    println!("Memory Delta:   {} bytes", mem_used);
    println!("Current Memory: {} bytes", usage_after.physical_mem);

    // --- Stress Test (Large Input) ---
    println!("\n[Stress Test - 1 000 * 5 elements]");

    let large_s = vec![
        (2..1_000).collect::<Vec<i32>>(),
        (2..1_000).collect::<Vec<i32>>(),
        (2..1_000).collect::<Vec<i32>>(),
        (2..1_000).collect::<Vec<i32>>(),
        (2..1_000).collect::<Vec<i32>>(),
    ];

    large_s.len();

    let usage_before_stress = memory_stats().unwrap();
    let start_time_stress = Instant::now();   

    let _ = sol::spiral_order(large_s.clone()); 

    let duration_stress = start_time_stress.elapsed();
    let usage_after_stress = memory_stats().unwrap();   

    let mem_used_stress = usage_after_stress
        .physical_mem
        .saturating_sub(usage_before_stress.physical_mem); 

    println!("Execution Time: {:?}", duration_stress);
    println!("Memory Delta:   {} bytes", mem_used_stress);
    println!("Current Memory: {} bytes", usage_after_stress.physical_mem);
}
