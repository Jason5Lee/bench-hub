using BenchmarkDotNet.Configs;
using BenchmarkDotNet.Diagnosers;
using BenchmarkDotNet.Running;
using System.Runtime.InteropServices;

var config = ManualConfig.Create(DefaultConfig.Instance);
// Only Windows and Linux are supported in DisassemblyDiagnoser without Mono.
if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ||
    RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
{
    config = config
        .AddDiagnoser(new DisassemblyDiagnoser(new DisassemblyDiagnoserConfig(printSource: true)));
}

BenchmarkSwitcher.FromAssembly(typeof(Program).Assembly).Run(args, config);
