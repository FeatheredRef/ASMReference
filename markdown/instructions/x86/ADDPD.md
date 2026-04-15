> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADDPD

Adds two packed double-precision floating-point values. The operation adds the two 64-bit double-precision floating-point numbers in the first operand to the two 64-bit double-precision floating-point numbers in the second operand and stores the result in the destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE3 extension set to be supported by the processor.

The operation is performed based on the rounding control and exception masking defined in the MXCSR register. If the result cannot be represented as a double-precision floating-point number, an exception may be triggered.

- **Alignment**: Memory operands MUST be aligned on a 16-byte boundary unless the processor supports unaligned memory access; otherwise, a general-protection exception may occur.
- **Floating-point Exceptions**: The instruction may set the following flags in MXCSR: #D, #O, #U, #P.