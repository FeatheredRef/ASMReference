> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVNTI

Moves a dword from a register to a memory destination using a non-temporal store, bypassing the cache hierarchy to avoid cache pollution.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| r32 | m4 |
| imm | #I |
| m4 | #I |

DO NOT support LOCK

This instruction is only available in 32-bit mode or when the processor is operating in compatibility mode. It SHALL NOT be used in 64-bit mode; for 64-bit operations, `MOVNTI` is replaced by `MOVNTDQ` or `MOVNTPD`.

The store is weakly ordered. To ensure the data is visible to other processors or devices, a memory fence instruction (such as `SFENCE`) MUST be executed. Failure to use a fence may result in the processor committing the write to memory after subsequent instructions have executed.