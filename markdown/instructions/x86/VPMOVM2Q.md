> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVM2Q

Moves data from a 128-bit vector register to a 64-bit general-purpose register, performing a zero-extension of the lowest 32 bits of the source operand if the source is 32-bit, or simply moving the lowest 64 bits of the source vector to the destination register.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | r64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The destination register r64 is completely overwritten. The upper 64 bits of the source xmm register are ignored. When executing this instruction, the programmer SHALL ensure the processor supports the AVX instruction set extensions to avoid an invalid opcode exception.