> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTESTNMB

Performs a bitwise logical AND operation between two mask registers and sets the ZF flag in the FLAGS register based on the result. If the result of the logical AND is non-zero, ZF is cleared; if the result is zero, ZF is set.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| k rN | #I |
| k rN | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 foundation.

The destination operand is an implicit destination as the instruction only modifies the flags register. Users SHALL ensure that the mask registers used are valid and within the supported bit-width of the architecture to avoid undefined behavior. Failure to target a 64-bit environment WILL result in an illegal instruction exception.