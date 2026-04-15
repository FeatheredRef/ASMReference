> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCHS

Changes the sign of the floating-point value in the destination operand to positive.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f80 | f80 |
| f32 | f32 |
| f64 | f64 |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It operates exclusively on the x87 FPU register stack.

The instruction modifies the sign bit of the floating-point representation to 0. It SHALL NOT affect the exponent or the significand of the operand. No floating-point exceptions are generated.