# Benchmarks of Rust

## Result

### Interesting observations

- Apple M4 hardware has a high CAS failure cost, while Intel CAS failures are slightly faster than CAS successes. Intel's results align with expectations (a slight decrease in success compared to writes), and the cause of Apple M4's slowness is currently unknown.
- Tokio's try_send has the best performance among async channels. However, even without blocking, the send operation is significantly slower.

### Apple M4 Pro

seqcst_success time: [988.37 ps 988.95 ps 989.58 ps]

acqrel_success time: [989.21 ps 991.16 ps 994.30 ps]

seqcst_failure time: [11.244 ns 11.390 ns 11.550 ns]

acqrel_failure time: [11.278 ns 11.423 ns 11.580 ns]

acqrel_weak_failure time: [11.295 ns 11.460 ns 11.631 ns]

crossbeam_try_then_send_blocking time: [4.2073 ns 4.2197 ns 4.2312 ns]

crossbeam_send_blocking time: [4.4819 ns 4.4964 ns 4.5100 ns]

std_try_then_send_blocking time: [4.0300 ns 4.0372 ns 4.0440 ns]

std_send_blocking time: [4.2389 ns 4.2512 ns 4.2624 ns]

async_channel_try_then_send_blocking time: [16.585 ns 16.636 ns 16.684 ns]

async_channel_send_blocking time: [17.325 ns 17.380 ns 17.430 ns]

async_channel_try_then_send_await time: [16.590 ns 16.644 ns 16.694 ns]

async_channel_send_await time: [17.140 ns 17.191 ns 17.239 ns]

futures_try_then_send_await time: [16.349 ns 16.420 ns 16.491 ns]

futures_send_await time: [19.477 ns 19.573 ns 19.667 ns]

tokio_try_then_send_blocking time: [6.4019 ns 6.4311 ns 6.4596 ns]

tokio_send_blocking time: [21.028 ns 21.088 ns 21.145 ns]

tokio_try_then_send_await time: [6.6342 ns 6.6679 ns 6.6988 ns]

tokio_send_await time: [12.630 ns 12.673 ns 12.711 ns]

Deserialize Item time: [2.0158 ms 2.0209 ms 2.0258 ms]

Deserialize SimplerItem time: [767.51 µs 772.60 µs 777.68 µs]

### Intel i7-13700H on Windows 11

seqcst_success time: [8.5420 ns 8.6923 ns 8.8621 ns]

acqrel_success time: [8.7932 ns 8.9822 ns 9.1789 ns]

seqcst_failure time: [6.4465 ns 6.5739 ns 6.7051 ns]

acqrel_failure time: [6.2532 ns 6.3726 ns 6.5056 ns]

acqrel_weak_failure time: [6.2128 ns 6.3075 ns 6.4266 ns]

crossbeam_try_then_send_blocking time: [13.628 ns 13.894 ns 14.215 ns]

crossbeam_send_blocking time: [13.717 ns 14.050 ns 14.456 ns]

std_try_then_send_blocking time: [14.823 ns 15.297 ns 15.814 ns]

std_send_blocking time: [15.298 ns 15.602 ns 15.923 ns]

async_channel_try_then_send_blocking time: [59.403 ns 60.754 ns 62.431 ns]

async_channel_send_blocking time: [58.042 ns 58.433 ns 58.940 ns]

async_channel_try_then_send_await time: [57.899 ns 58.744 ns 59.729 ns]

async_channel_send_await time: [64.037 ns 65.338 ns 66.718 ns]

futures_try_then_send_await time: [80.376 ns 82.961 ns 85.449 ns]

futures_send_await time: [72.346 ns 73.963 ns 75.730 ns]

tokio_try_then_send_blocking time: [44.301 ns 45.291 ns 46.522 ns]

tokio_send_blocking time: [86.276 ns 88.072 ns 90.226 ns]

tokio_try_then_send_await time: [44.956 ns 46.584 ns 48.344 ns]

tokio_send_await time: [77.347 ns 79.216 ns 81.285 ns]

Deserialize Item time: [3.6985 ms 3.7772 ms 3.8589 ms]

Deserialize SimplerItem time: [1.0710 ms 1.0890 ms 1.1088 ms]
