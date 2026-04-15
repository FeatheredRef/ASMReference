> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTESTMW

Performs a bitwise logical AND between two word-sized operands and sets the ZF flag in the EFLAGS register based on the result. If the result of the AND operation is non-zero, ZF is cleared; otherwise, ZF is set.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | #I |
| m2 | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is not supported in compatibility mode.

The instruction operates on the low 16 bits of the source operands. When using YMM or ZMM registers, the operation is performed on the first word element of the vector; higher elements are ignored and do not affect the flags. Failure to ensure the correct register size or alignment in memory may result in a general protection fault.