> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PREFETCHW

Prefetches data into the cache with the intent to write, marking the cache line as modified in the cache.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | mN |
| imm | #I |
| memory | #I |

DO NOT support LOCK

The instruction is a hint to the processor; the processor MAY ignore the request. It does not trigger a fault if the memory address is invalid. It is available in 64-bit mode and compatibility mode.

The instruction MUST NOT be used to synchronize memory between threads. Since it is a non-binding hint, the software MUST NOT rely on the data being present in the cache immediately after the instruction executes. Using this instruction on memory regions that are not mapped or are protected SHOULD NOT result in an exception.