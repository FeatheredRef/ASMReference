> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ROUNDPS

Rounds four packed single-precision floating-point values according to the rounding mode specified in the immediate operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It REQUIRES the SSE4.1 instruction set extension.

The rounding mode is encoded in the immediate byte; any value other than the four defined rounding modes (00, 01, 10, 11) SHALL result in an invalid opcode exception. The instruction ignores the rounding control bits in the MXCSR register, overriding them with the immediate value.

If the result cannot be represented as a single-precision floating-point number, #O or #U may be signaled. If the result is inexact, #P SHALL be signaled. Ensure that the destination register is not used as a source if the operation is not meant to be destructive, although the xmm register is typically used for both in the non-memory variant.