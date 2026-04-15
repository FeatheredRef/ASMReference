> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ANDNPS

Performs a bitwise AND operation between two packed single-precision floating-point values, but with the second operand (the source) bitwise inverted (NOTed). The result is then stored in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE3 instruction set extension; if the processor does not support SSE3, the execution of this instruction SHALL result in an invalid opcode exception.

The memory operand MUST be 16-byte aligned. If the memory address is not aligned to a 16-byte boundary, a general-protection exception SHALL occur. This instruction does not affect the floating-point status register or the EFLAGS register.