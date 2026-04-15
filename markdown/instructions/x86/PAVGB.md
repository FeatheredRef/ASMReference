> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PAVGB

Computes the average of two packed floating-point values. The instruction adds the two source operands and multiplies the result by 0.5.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode or compatibility mode. It REQUIRES support for the AVX or AVX-512 instruction sets depending on the register width used.

To avoid precision loss or unexpected results, the user SHALL ensure that the input values are valid floating-point numbers; if either input is a NaN, the result SHALL be a NaN. The operation is subject to the rounding control in the MXCSR register.