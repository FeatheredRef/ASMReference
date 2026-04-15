> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ORPS

Performs a bitwise logical OR operation on two single-precision floating-point values. The result is stored in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE support feature to be enabled in the processor.

The operation is performed on the raw bit patterns of the floating-point values; the processor DOES NOT treat the operands as numeric values. Consequently, floating-point exceptions such as #D, #Z, #O, #U, and #P are NOT triggered. The result is not rounded.