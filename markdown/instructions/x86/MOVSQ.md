> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSQ

Copies a qword from the memory location pointed to by RSI to the memory location pointed to by RDI.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | m8 |
| #I | #I |

DO NOT support LOCK

The instruction is only available in 64-bit mode. It requires the use of the RSI and RDI registers as implicit pointers; any attempt to use other registers for the source or destination addresses is not supported.

If the Direction Flag (DF) is set, RSI and RDI are decremented after the transfer; if DF is clear, they are incremented. Failure to initialize the DF via `CLD` or `STD` may result in unintended memory access patterns. This instruction performs a memory-to-memory copy, which is not atomic.