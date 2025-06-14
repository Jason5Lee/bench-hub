using System.Runtime.CompilerServices;

namespace Jason5Lee.MicroBenchmarks.CSharp;

public static class GenericExtensions
{
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static TResult Let<T, TResult>(this T source, Func<T, TResult> func) => func(source);
}
