#[cfg(feature = "bench")]
macro_rules! benchmark {
    ($benchmark_name:expr, $code_block:block) => {{
        let start = std::time::Instant::now();
        let result = $code_block;
        println!("Benchmark: [{}] {:?}", $benchmark_name, start.elapsed());
        result
    }};
}

#[cfg(not(feature = "bench"))]
macro_rules! benchmark {
    ($benchmark_name:expr, $code_block:block) => {{
        $code_block
    }};
}
