> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PREFETCHh

This instruction provides a hint to the processor to fetch data from the specified memory address into the cache hierarchy. It is intended to reduce cache miss latency by pre-loading data before it is explicitly requested by a subsequent load instruction.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | #I |
| reg | #I |
| imm | #I |

DO NOT support LOCK

The `PREFETCHh` instruction is a hint; the processor MAY ignore the request based on internal resource availability or power management policies. It does not trigger a fault if the memory address is invalid or inaccessible, meaning it SHALL NOT cause an architectural exception.

The instruction is available in 64-bit mode and compatibility mode. It is not supported in 32-bit mode.

Since the instruction is non-faulting, developers SHOULD NOT use it to probe for the validity of a memory address. Using `PREFETCHh` on an address that is not aligned to a cache line boundary will result in the processor fetching the entire cache line containing that address.