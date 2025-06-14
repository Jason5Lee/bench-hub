//! This benchmark tests the most efficient sending method for a Channel when sending is non-blocking. The implementation used in the test considers Channel blocking, but in reality, there is no blocking. This scenario is common when the capacity is sufficiently large.

// benches/receive_bench.rs

use std::time::Instant;

use criterion::{Criterion, criterion_group, criterion_main};
use futures::channel::mpsc as futures_channel;
use futures::executor::block_on;
use futures::sink::SinkExt;
use tokio::sync::mpsc as tokio_channel;

macro_rules! bench_try_then_send_blocking {
    ($group:ident, $channel_name:literal, $creation:expr, $send_blocking_method:ident, $full_error_branch:path) => {
        $group.bench_function(concat!($channel_name, "_try_then_send_blocking"), |b| {
            b.iter_custom(|iters| {
                let n = iters as usize;
                let (s, _r) = $creation(n);

                let start = Instant::now();
                for i in 0..n {
                    match s.try_send(i as u128) {
                        Ok(_) => {}
                        Err(err) => match err {
                            $full_error_branch(elem) => s.$send_blocking_method(elem).unwrap(),
                            _ => unreachable!(),
                        },
                    }
                }
                start.elapsed()
            })
        });
    };
}

macro_rules! bench_send_blocking {
    ($group:ident, $channel_name:literal, $creation:expr, $send_blocking_method:ident) => {
        $group.bench_function(concat!($channel_name, "_send_blocking"), |b| {
            b.iter_custom(|iters| {
                let n = iters as usize;
                let (s, _r) = $creation(n);

                let start = Instant::now();
                for i in 0..n {
                    s.$send_blocking_method(i).unwrap();
                }
                start.elapsed()
            })
        });
    };
}

macro_rules! bench_try_then_send_await {
    ($group:ident, $channel_name:literal, $creation:expr, $send_method:ident, $full_error_branch:path) => {
        $group.bench_function(concat!($channel_name, "_try_then_send_await"), |b| {
            b.iter_custom(|iters| {
                let n = iters as usize;
                let (s, _r) = $creation(n);

                let start = Instant::now();
                block_on(async {
                    for i in 0..n {
                        match s.try_send(i as u128) {
                            Ok(_) => {}
                            Err(err) => match err {
                                $full_error_branch(elem) => s.$send_method(elem).await.unwrap(),
                                _ => unreachable!(),
                            },
                        }
                    }
                });
                start.elapsed()
            })
        });
    };
}

macro_rules! bench_send_await {
    ($group:ident, $channel_name:literal, $creation:expr, $send_method:ident) => {
        $group.bench_function(concat!($channel_name, "_send_await"), |b| {
            b.iter_custom(|iters| {
                let n = iters as usize;
                #[allow(unused_mut)] // futures channel requires while others don't
                let (mut s, _r) = $creation(n);

                let start = Instant::now();
                block_on(async {
                    for i in 0..n {
                        s.$send_method(i as u128).await.unwrap();
                    }
                });
                start.elapsed()
            })
        });
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_try_then_send_blocking!(
        c,
        "crossbeam",
        crossbeam_channel::bounded,
        send,
        crossbeam_channel::TrySendError::Full
    );
    bench_send_blocking!(c, "crossbeam", crossbeam_channel::bounded, send);

    bench_try_then_send_blocking!(
        c,
        "std",
        std::sync::mpsc::sync_channel,
        send,
        std::sync::mpsc::TrySendError::Full
    );
    bench_send_blocking!(c, "std", std::sync::mpsc::sync_channel, send);

    bench_try_then_send_blocking!(
        c,
        "async_channel",
        async_channel::bounded,
        send_blocking,
        async_channel::TrySendError::Full
    );
    bench_send_blocking!(c, "async_channel", async_channel::bounded, send_blocking);
    bench_try_then_send_await!(
        c,
        "async_channel",
        async_channel::bounded,
        send,
        async_channel::TrySendError::Full
    );
    bench_send_await!(c, "async_channel", async_channel::bounded, send);

    // Futures' channel has unique errors
    c.bench_function("futures_try_then_send_await", |b| {
        b.iter_custom(|iters| {
            let n = iters as usize;
            let (mut s, _r) = futures_channel::channel::<u128>(n);

            let start = Instant::now();
            block_on(async {
                for i in 0..n {
                    match s.try_send(i as u128) {
                        Ok(_) => {}
                        Err(err) => {
                            if err.is_full() {
                                s.send(err.into_inner()).await.unwrap();
                            } else {
                                unreachable!();
                            }
                        }
                    }
                }
            });
            start.elapsed()
        })
    });
    bench_send_await!(c, "futures", futures_channel::channel, send);

    bench_try_then_send_blocking!(
        c,
        "tokio",
        tokio_channel::channel,
        blocking_send,
        tokio_channel::error::TrySendError::Full
    );
    bench_send_blocking!(c, "tokio", tokio_channel::channel, blocking_send);
    bench_try_then_send_await!(
        c,
        "tokio",
        tokio_channel::channel,
        send,
        tokio_channel::error::TrySendError::Full
    );
    bench_send_await!(c, "tokio", tokio_channel::channel, send);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
