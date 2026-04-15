> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SCASW

Compares the value in the `ax` register with a word-sized value from the memory location pointed to by `edi` (in 32-bit mode) or `rdi` (in 64-bit mode). The `zf` flag is set if the operands are equal. After the comparison, the index register (`edi` or `rdi`) is incremented or decremented by 2, depending on the value of the `df` flag in the `rflags` register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg (ax) | m2 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates exclusively on 16-bit operands regardless of the current operating mode.

The `df` (Direction Flag) MUST be configured correctly before execution; if `df` is 0, the index register increments, and if `df` is 1, it decrements. Using this instruction with a `rep` prefix WILL continue the operation until `zf` is cleared or `rcx` reaches 0. Failure to initialize `rcx` with the correct count before a `rep` prefix WILL result in unintended memory accesses.