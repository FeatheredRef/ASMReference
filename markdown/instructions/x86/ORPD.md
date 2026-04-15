> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ORPD

Performs a bitwise logical OR operation on the 64-bit packed double-precision floating-point values of the source and destination operands.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m64 | xmm |
| #I | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE2 instruction set extension.

The instruction operates on the bit representation of the floating-point values; it does not treat the operands as numeric floating-point values. Consequently, no floating-point exceptions (#D, #Z, #O, #U, #P) are generated. User MUST ensure that the target xmm register is correctly aligned if using memory operands to avoid alignment check exceptions.