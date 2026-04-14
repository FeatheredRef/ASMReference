# SIMD

> Instructions that allow you to process multiple operations with a single instruction.

SIMD (Single Instruction, Multiple Data) is a CPU feature that is available on ARM through NEON, and in x86-64 through AVX — in x86-64, there is also SSE.

Generally, SIMD is more modernly used for processing vectors. Thus, using AVX or NEON allows you to process 16 bytes, 32 bytes, or 64 bytes of data in parallel with a single instruction, therefore increasing throughput. 

### Large width

It is worth mentioning that some SIMD instructions may have an activation cost, and that others may use a lot of power. This decreases CPU clock frequency due to either thermal throttling or power strain.

Even so, most of the ones that cause this throttling are good by themselves, as even though the instructions per second decrease, the byte throughput remains high. This is mostly the case with, for example, AVX-512.
