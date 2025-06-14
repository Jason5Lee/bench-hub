using BenchmarkDotNet.Attributes;

namespace Jason5Lee.MicroBenchmarks.CSharp;

public class BenchmarkPipeline
{
    private const int ITERATIONS = 1000;
    private const int SEED = 1742098347;
    private int initVal;
    private int factor;
    private int constant;
    private Random rand = new Random(SEED);

    [GlobalSetup]
    public void Setup()
    {
        initVal = rand.Next();
        factor = rand.Next();
        constant = rand.Next();
    }

    [Benchmark]
    public int DirectCall()
    {
        int result = initVal;
        for (int i = 0; i < ITERATIONS; ++i)
        {
            result = result * factor + constant;
        }
        
        return result;
    }

    [Benchmark]
    public int PipelineOnce()
    {
        int result = initVal;
        for (int i = 0; i < ITERATIONS; ++i)
        {
            result = result.Let(r => r * factor + constant);
        }

        return result;
    }

    [Benchmark]
    public int PipelineTwice()
    {
        int result = initVal;
        for (int i = 0; i < ITERATIONS; ++i)
        {
            result = result.Let(r => r * factor).Let(r => r + constant);
        }

        return result;
    }
}
