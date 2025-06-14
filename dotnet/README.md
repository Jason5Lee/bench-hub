# Benchmarks of .NET

## Results

### Apple M4 Pro

| Method        |       Mean |     Error |    StdDev |
| ------------- | ---------: | --------: | --------: |
| DirectCall    |   847.3 ns |   0.52 ns |   0.43 ns |
| PipelineOnce  | 4,019.5 ns |  55.45 ns |  51.87 ns |
| PipelineTwice | 9,429.8 ns | 131.54 ns | 123.05 ns |
