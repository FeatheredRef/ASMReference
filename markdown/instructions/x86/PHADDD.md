> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PHADDD

Adds two packed double-precision floating-point values from the source and destination operands and stores the result in the destination operand.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| m128 | xmm |
| xmm | xmm |
| imm | #I |

DO NOT support LOCK

This instruction SHALL ONLY be executed in 64-bit mode or compatibility mode. It is not supported in 32-bit mode.

The result SHALL be subject to the rounding control and exception masking defined in the MXCSR register. If the operation results in a value that cannot be represented in the target format, the processor SHALL trigger #O or #U according to the IEEE 754 standard.

Ensure that the xmm registers are aligned to 16-byte boundaries when using memory operands (m128) to avoid performance penalties or alignment check exceptions depending on the processor configuration. Improper alignment MAY result in a general protection fault (#GP) if alignment checking is enabled.