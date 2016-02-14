#![feature(test)]

extern crate crossbeam;
extern crate ipc_channel;
extern crate test;

use ipc_channel::platform;

#[bench]
fn bench_tiny_data(b: &mut test::Bencher) {
    let data: &[u8] = b"1234567";
    let (tx, rx) = platform::channel().unwrap();

    b.iter(|| {
        tx.send(data, vec![], vec![]).unwrap();
        rx.recv().unwrap()
    });
}

#[bench]
fn bench_medium_data(b: &mut test::Bencher) {
    let data: Vec<u8> = (0..65536).map(|i| (i % 251) as u8).collect();
    let data: &[u8] = &data[..];
    let (tx, rx) = platform::channel().unwrap();

    b.iter(|| {
        tx.send(data, vec![], vec![]).unwrap();
        rx.recv().unwrap()
    });
}

#[bench]
fn bench_big_data(b: &mut test::Bencher) {
    let data: Vec<u8> = (0.. 1024 * 1024).map(|i| (i % 251) as u8).collect();
    let data: &[u8] = &data[..];
    let (tx, rx) = platform::channel().unwrap();

    b.iter(|| {
        crossbeam::scope(|scope| {
            scope.spawn(|| {
                tx.send(data, vec![], vec![]).unwrap();
            });
            rx.recv().unwrap()
        });
    });
}
