> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XORPD

Performs a bitwise logical XOR operation on two double-precision floating-point values.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE2 support.

The destination operand MUST NOT be the same as the source operand if memory aliasing occurs. Since this instruction operates on 64-bit floating-point values, ensuring proper alignment of m8 operands is REQUIRED to avoid performance penalties or general protection faults depending on the processor configuration.

If the memory operand is not aligned on an 8-byte boundary, the operation MAY incur a performance penalty. This instruction does NOT affect the floating-point status flags.