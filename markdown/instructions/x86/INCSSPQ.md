> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INCSSPQ

Increments the unsigned quadword value at the specified destination by one.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| implied | r64 |
| implied | m8 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The instruction does not affect any EFLAGS bits. Because it does not update the flags, it cannot be used as a basis for conditional branching.

If the destination is a memory operand, the operation is not atomic unless specifically handled by the memory model; however, since the LOCK prefix is not supported for this specific mnemonic, concurrent access to the same memory location by multiple processors may lead to race conditions.