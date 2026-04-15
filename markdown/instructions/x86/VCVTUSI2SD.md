> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUSI2SD

Converts an unsigned integer source to a double-precision floating-point destination.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m64 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE3 instruction set extension.

The destination register is overwritten with the converted value; therefore, the original value in the destination register is lost. If the source is a memory operand, the value is read as a u64. If the source is an XMM register, only the lower 64 bits are used.

To avoid precision loss or unexpected behavior, ensure the source is treated as an unsigned 64-bit integer. Using this instruction on signed data will result in an incorrect floating-point representation. Precision exceptions (#P) may occur if the result cannot be represented exactly in the f64 format.