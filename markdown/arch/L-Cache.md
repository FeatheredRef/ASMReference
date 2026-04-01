# L-Cache

> L-Cache is a region in-chips, or nearby chips, mitigating latency between [memory](</arch/memory>). 

Around the 90s, when CPUs started to develop, becoming more advanced, complex, efficient, and faster, one problem started to arise. Even while the chips were becoming faster, they started to spend time waiting for data to arrive from RAM more than theoretically necessary. This became a problem, as if your CPU runs one million instructions a second, it would spend a significant time waiting data to arrive from RAM. 

This cache being the L-Cache, essentially, a memory region that is on-chip, or very near to it. When the instructions fetch a range of memory repeatedly, or the CPU predicts future usage, the chip pulls this data closer — A cache layer.

There are three layers (usually), L3, L2, and L1. Each being farther from the chip the larger the number. The L3 cache size is consistently the largest. Result of all cores of a CPU usually sharing it, as it is farther the chip can afford the larger cache. L2 is akin to L3, being closer, still decently large, but not shared across cores.

L1, being the smallest cache, it is the fastest. Usually, the CPU will use it on data that is being used a lot. For instance, if you are doing a "sum" over a contiguous range of numbers, this might be a predictable workload for CPUs, therefore it may fetch the data to L1 before the instructions even issue it.

As the fetch can be preemptive, the CPU may simply fail the prediction. This being called a cache miss, when this happens, the CPU must fetch the data from the upper layer, worst case fetching from RAM. This process is fast, even so, it takes time that a performance-intensive workload would like to avoid wasting.

Worth noting, that even while algorithms may be O(n log n), or O(1), it may be slower than an O(n²) if the bottleneck is memory-wise. One of the implementations that is aware of that is quicksort, which switches to insertion sort on small Ns.

Also, in some chips you are capable of pre-fetching the data to a cache layer. In x86-64 you can do so with the instruction `prefetchN` where N is the desired layer (either 0, 1, or 2).

In conclusion, L-Cache is a memory in the chip, or very near the chip, that amortizes the latency between CPU and [memory](</arch/memory>).

Visualization for comparison purposes, even while the L2 Cache is accurate, the others are not exact. Don't use the data in this table for something serious.



| Model                | L1 Cache (per core) | L2 Cache (total) | L3 Cache (shared) |
|---------------------|--------------------|------------------|-------------------|
| [Intel Core i3-14100](<https://www.intel.com.br/content/www/br/pt/products/sku/236774/intel-core-i3-processor-14100-12m-cache-up-to-4-70-ghz/specifications.html>) | 64 KB              | 5 MB             | 12 MB             |
| [Intel Core i5-14600K](<https://www.intel.com.br/content/www/br/pt/products/sku/236799/intel-core-i5-processor-14600k-24m-cache-up-to-5-30-ghz/specifications.html>)| 64 KB              | 20 MB            | 24 MB             |
| [Intel Core i7-14700K](<https://www.intel.com.br/content/www/br/pt/products/sku/236783/intel-core-i7-processor-14700k-33m-cache-up-to-5-60-ghz/specifications.html>)| 64 KB              | 28 MB            | 33 MB             |
| [Intel Core i9-14900K](<https://www.intel.com.br/content/www/br/pt/products/sku/236773/intel-core-i9-processor-14900k-36m-cache-up-to-6-00-ghz/specifications.html>)| 64 KB              | 32 MB            | 36 MB             |
| [Intel Xeon 6780E](<https://www.intel.com.br/content/www/br/pt/products/sku/240362/intel-xeon-6780e-processor-108m-cache-2-20-ghz/specifications.html>)    | 64 KB              | ~160 MB          | 336 MB            |
