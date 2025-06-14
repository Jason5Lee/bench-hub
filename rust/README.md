# Benchmarks of Rust

## Result

### Apple M4 Pro

seqcst_success time: [988.37 ps 988.95 ps 989.58 ps]
Found 10 outliers among 100 measurements (10.00%)
1 (1.00%) low mild
4 (4.00%) high mild
5 (5.00%) high severe

acqrel_success time: [989.21 ps 991.16 ps 994.30 ps]
Found 11 outliers among 100 measurements (11.00%)
3 (3.00%) high mild
8 (8.00%) high severe

seqcst_failure time: [11.244 ns 11.390 ns 11.550 ns]

acqrel_failure time: [11.278 ns 11.423 ns 11.580 ns]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild

acqrel_weak_failure time: [11.295 ns 11.460 ns 11.631 ns]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild

     Running benches/channel_nonblocking_send.rs (target/release/deps/channel_nonblocking_send-66ef1802c46b15e2)

Gnuplot not found, using plotters backend
Benchmarking crossbeam_try_then_send_blocking: Collecting 100 samples in estimated 5.0000 s (702M i
crossbeam_try_then_send_blocking
time: [4.2073 ns 4.2197 ns 4.2312 ns]
Found 2 outliers among 100 measurements (2.00%)
1 (1.00%) low severe
1 (1.00%) high severe

Benchmarking crossbeam_send_blocking: Collecting 100 samples in estimated 5.0000 s (829M iterations
crossbeam_send_blocking time: [4.4819 ns 4.4964 ns 4.5100 ns]
Found 3 outliers among 100 measurements (3.00%)
1 (1.00%) low severe
1 (1.00%) low mild
1 (1.00%) high severe

Benchmarking std_try_then_send_blocking: Collecting 100 samples in estimated 5.0000 s (579M iterati
std_try_then_send_blocking
time: [4.0300 ns 4.0372 ns 4.0440 ns]
Found 4 outliers among 100 measurements (4.00%)
3 (3.00%) low severe
1 (1.00%) high severe

std_send_blocking time: [4.2389 ns 4.2512 ns 4.2624 ns]
Found 3 outliers among 100 measurements (3.00%)
1 (1.00%) low severe
1 (1.00%) low mild
1 (1.00%) high severe

Benchmarking async_channel_try_then_send_blocking: Collecting 100 samples in estimated 5.0000 s (25
async_channel_try_then_send_blocking
time: [16.585 ns 16.636 ns 16.684 ns]
Found 2 outliers among 100 measurements (2.00%)
1 (1.00%) low severe
1 (1.00%) low mild

Benchmarking async_channel_send_blocking: Collecting 100 samples in estimated 5.0000 s (260M iterat
async_channel_send_blocking
time: [17.325 ns 17.380 ns 17.430 ns]
Found 2 outliers among 100 measurements (2.00%)
2 (2.00%) low mild

Benchmarking async_channel_try_then_send_await: Collecting 100 samples in estimated 5.0000 s (253M
async_channel_try_then_send_await
time: [16.590 ns 16.644 ns 16.694 ns]
Found 2 outliers among 100 measurements (2.00%)
1 (1.00%) low severe
1 (1.00%) low mild

Benchmarking async_channel_send_await: Collecting 100 samples in estimated 5.0000 s (245M iteration
async_channel_send_await
time: [17.140 ns 17.191 ns 17.239 ns]
Found 3 outliers among 100 measurements (3.00%)
1 (1.00%) low severe
1 (1.00%) low mild
1 (1.00%) high mild

Benchmarking futures_try_then_send_await: Collecting 100 samples in estimated 5.0002 s (72M iterati
futures_try_then_send_await
time: [16.349 ns 16.420 ns 16.491 ns]
Found 5 outliers among 100 measurements (5.00%)
3 (3.00%) high mild
2 (2.00%) high severe

futures_send_await time: [19.477 ns 19.573 ns 19.667 ns]
Found 3 outliers among 100 measurements (3.00%)
2 (2.00%) high mild
1 (1.00%) high severe

Benchmarking tokio_try_then_send_blocking: Collecting 100 samples in estimated 5.0000 s (317M itera
tokio_try_then_send_blocking
time: [6.4019 ns 6.4311 ns 6.4596 ns]
Found 9 outliers among 100 measurements (9.00%)
5 (5.00%) low mild
4 (4.00%) high mild

tokio_send_blocking time: [21.028 ns 21.088 ns 21.145 ns]
Found 3 outliers among 100 measurements (3.00%)
1 (1.00%) low severe
2 (2.00%) low mild

Benchmarking tokio_try_then_send_await: Collecting 100 samples in estimated 5.0000 s (311M iteratio
tokio_try_then_send_await
time: [6.6342 ns 6.6679 ns 6.6988 ns]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high severe

tokio_send_await time: [12.630 ns 12.673 ns 12.711 ns]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild

     Running benches/serde_json_optimize_fields.rs (target/release/deps/serde_json_optimize_fields-e4c069e8a4cc82ba)

Gnuplot not found, using plotters backend
Deserialize Item time: [2.0158 ms 2.0209 ms 2.0258 ms]

Deserialize SimplerItem time: [767.51 µs 772.60 µs 777.68 µs]
