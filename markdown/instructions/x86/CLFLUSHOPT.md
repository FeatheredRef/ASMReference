> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLFLUSHOPT

Invalidates the cache line containing the specified linear address. This instruction is an optimized version of CLFLUSH; it provides a hint to the processor that the flush operation can be performed more efficiently by allowing the operation to be unordered with respect to other CLFLUSHOPT instructions.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | #I |

DO NOT support LOCK

The instruction SHALL only be executed if the processor supports the CLFLUSHOPT feature flag. It operates on a linear address specified by a memory operand. The instruction is available in 64-bit mode and compatibility mode.

To avoid data inconsistency or loss, the software MUST use a memory fence instruction (such as SFENCE) if ordering is required between the flush operation and subsequent memory accesses, as CLFLUSHOPT is weakly ordered. Failure to do so MAY result in the processor executing subsequent stores before the cache line is actually invalidated.