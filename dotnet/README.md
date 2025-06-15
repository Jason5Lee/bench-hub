# Benchmarks of .NET

## Results

### Apple M4 Pro

| Method        |       Mean |     Error |    StdDev |
| ------------- | ---------: | --------: | --------: |
| DirectCall    |   847.3 ns |   0.52 ns |   0.43 ns |
| PipelineOnce  | 4,019.5 ns |  55.45 ns |  51.87 ns |
| PipelineTwice | 9,429.8 ns | 131.54 ns | 123.05 ns |

### Intel i7-13700H on Windows 11

| Method        | Mean       | Error     | StdDev    | Code Size |
|-------------- |-----------:|----------:|----------:|----------:|
| DirectCall    |   779.8 ns |   2.99 ns |   2.80 ns |      26 B |
| PipelineOnce  | 3,071.8 ns |  25.30 ns |  22.43 ns |     116 B |
| PipelineTwice | 8,605.8 ns | 110.37 ns | 103.24 ns |     191 B |
