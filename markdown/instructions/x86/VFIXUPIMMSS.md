> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFIXUPIMMSS

This instruction converts a scalar single-precision floating-point value to the nearest floating-point value that can be represented in a specified precision, using a specified rounding mode. The operation is performed on the source operand using an immediate value to define the target precision and rounding mode.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode and 32-bit mode. It requires AVX-512 support.

The target precision is specified by the immediate operand. If the target precision is set to a value that exceeds the source precision, the value remains unchanged. The rounding mode is determined by the immediate operand, overriding the rounding control in the MXCSR register.

To avoid unexpected results, ensure the target register is not used for other active calculations as the instruction modifies the destination register in-place. Failure to specify a valid immediate value for precision or rounding may result in an #I exception.