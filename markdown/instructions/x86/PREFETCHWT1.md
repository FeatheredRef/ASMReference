> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PREFETCHWT1

Fetches data from the specified memory address into the cache and indicates that the data is likely to be written to, prompting the cache to fetch the data into the cache while minimizing cache pollution by using a lower priority (T1) replacement policy.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | #I |
| reg | #I |
| imm | #I |

DO NOT support LOCK

The instruction is a hint to the processor; therefore, it SHALL NOT be used to guarantee that data is present in the cache. It is supported in 64-bit mode and compatibility mode.

The instruction DOES NOT trigger a page fault if the memory address is invalid or not present; it is ignored if the address is not canonical or if there is a memory access violation. Because it is a hint, the processor MAY ignore the request entirely based on current system load or cache state.