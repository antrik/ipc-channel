#![feature(test)]

extern crate crossbeam;
extern crate ipc_channel;
extern crate test;

use ipc_channel::platform;

use std::sync::{mpsc, Mutex};

/// Allows doing multiple inner iterations per bench.iter() run.
///
/// This is mostly to amortise the overhead of spawning a thread in the benchmark
/// when sending larger messages (that might be fragmented).
///
/// Note that you need to compensate the displayed results
/// for the proportionally longer runs yourself,
/// as the benchmark framework doesn't know about the inner iterations...
const ITERATIONS: usize = 1;

macro_rules! create_bench_for_size {
    ($name:ident, $size:expr) => (
        #[bench]
        fn $name(b: &mut test::Bencher) {
            let data: Vec<u8> = (0..$size).map(|i| (i % 251) as u8).collect();
            let (tx, rx) = platform::channel().unwrap();

            let (wait_tx, wait_rx) = mpsc::channel();
            let wait_rx = Mutex::new(wait_rx);

            if $size > tx.get_max_fragment_size().unwrap() {
                b.iter(|| {
                    crossbeam::scope(|scope| {
                        scope.spawn(|| {
                            let wait_rx = wait_rx.lock().unwrap();
                            for _ in 0..ITERATIONS {
                                tx.send(&data, vec![], vec![]).unwrap();
                                if ITERATIONS > 1 {
                                    // Prevent beginning of the next send
                                    // from overlapping with receive of last fragment,
                                    // as otherwise results of runs with a large tail fragment
                                    // are significantly skewed.
                                    wait_rx.recv().unwrap();
                                }
                            }
                        });
                        for _ in 0..ITERATIONS {
                            rx.recv().unwrap();
                            if ITERATIONS > 1 {
                                wait_tx.send(()).unwrap();
                            }
                        }
                        // For reasons mysterious to me,
                        // not returning a value *from every branch*
                        // adds some 100 ns or so of overhead to all results --
                        // which is quite significant for very short tests...
                        0
                    })
                });
            } else {
                b.iter(|| {
                    for _ in 0..ITERATIONS {
                        tx.send(&data, vec![], vec![]).unwrap();
                        rx.recv().unwrap();
                    }
                    0
                });
            }
        }
    )
}

// It turns out the results have some crazy jumps between sizes,
// probably related to some allocator alignment stuff.
// What's more, these fluctuate strongly in seemingly random ways
// in response to various changes to the test setup etc.
//
// Warming up the memory somewhat mitigates these issues.
// The specific size used here was determined empirically
// to produce comparatively sane results -- on my system at least.
// (Maybe because it doesn't align well with 2's complements...
// Or maybe it's just an entirely random effect.)
//
// There might be more elegant and/or more effective ways to do the warm-up,
// using random allocations or something along these lines...
// However, I don't think it's really *that* important --
// and I already spent way too much time trying to figure this out :-(
create_bench_for_size!(_invoke_chaos, 777_777); // 111-up the beast ;-)

create_bench_for_size!(size_00_1, 1);
create_bench_for_size!(size_01_2, 2);
create_bench_for_size!(size_02_4, 4);
create_bench_for_size!(size_03_8, 8);
create_bench_for_size!(size_04_16, 16);
create_bench_for_size!(size_05_32, 32);
create_bench_for_size!(size_06_64, 64);
create_bench_for_size!(size_07_128, 128);
create_bench_for_size!(size_08_256, 256);
create_bench_for_size!(size_09_512, 512);
create_bench_for_size!(size_10_1k, 1 * 1024);
create_bench_for_size!(size_11_2k, 2 * 1024);
create_bench_for_size!(size_12_4k, 4 * 1024);
create_bench_for_size!(size_13_8k, 8 * 1024);
create_bench_for_size!(size_14_16k, 16 * 1024);
create_bench_for_size!(size_15_32k, 32 * 1024);
create_bench_for_size!(size_16_64k, 64 * 1024);
create_bench_for_size!(size_17_128k, 128 * 1024);
create_bench_for_size!(size_18_256k, 256 * 1024);
create_bench_for_size!(size_19_512k, 512 * 1024);
create_bench_for_size!(size_20_1m, 1 * 1024 * 1024);
create_bench_for_size!(size_21_2m, 2 * 1024 * 1024);
create_bench_for_size!(size_22_4m, 4 * 1024 * 1024);
create_bench_for_size!(size_23_8m, 8 * 1024 * 1024);
