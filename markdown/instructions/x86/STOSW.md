> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STOSW

Stores a word value from the `ax` register into the memory location pointed to by the destination index register. The destination index is then incremented or decremented by 2 bytes, depending on the value of the direction flag (DF).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r16 (ax) | m2 |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it specifically operates on a word (2 bytes). In x86-64, the destination index register is determined by the address size; for `STOSW`, the destination is typically `rdi`.

When using the `REP` prefix, the instruction SHALL repeat until `rcx` reaches 0. If `rcx` is initially 0, the instruction SHALL NOT execute the store operation.

The destination memory address MUST be aligned to a 2-byte boundary to avoid performance penalties or exceptions in specific alignment-checking environments. If the direction flag (DF) is set, the destination index is decremented; otherwise, it is incremented. Failure to correctly set the DF before a `REP STOSW` operation SHALL result in memory corruption of the unintended address range.