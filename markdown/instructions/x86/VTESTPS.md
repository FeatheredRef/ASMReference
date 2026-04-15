> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VTESTPS

Performs a bitwise logical AND operation on the lower 128 bits of two packed single-precision floating-point values and updates the floating-point status register flags (ZF, CF, PF) based on the result. The result of the logical AND is not stored; only the flags are affected.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension. It does not operate in compatibility mode if the processor does not support the required extensions.

The instruction does not modify the destination register; it only reads from the source operands to set the flags. Users MUST NOT expect the result of the AND operation to be preserved in any register. Since this instruction modifies the EFLAGS register, it MUST be accounted for in out-of-order execution or compiler optimizations to avoid race conditions with subsequent conditional jumps.