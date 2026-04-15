> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2PSX

Converts packed half-precision floating-point values to packed single-precision floating-point values with a specified rounding mode.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |
| imm | #I |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode. The instruction requires AVX-512 FP16 extensions to be enabled.

The rounding mode is controlled by the immediate operand, which overrides the rounding control in the MXCSR register. Failure to provide a valid immediate value results in an invalid operation. Precision (#P) and Underflow (#U) exceptions may be triggered based on the conversion result and the active rounding mode.