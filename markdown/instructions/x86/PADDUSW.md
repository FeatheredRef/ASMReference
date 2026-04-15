> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PADDUSW

Adds unsigned word values from two XMM registers, saturating the result to the unsigned word maximum (65535) if an overflow occurs.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The operation is performed on packed 16-bit unsigned integers. If the addition of two u16 values exceeds 65535, the result is set to 65535. Failure to ensure that the input data is treated as unsigned u16 will result in incorrect saturation behavior, as the instruction does not perform sign extension.