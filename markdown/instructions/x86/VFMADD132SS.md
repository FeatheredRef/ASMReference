> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD132SS

Multiplies a scalar single-precision floating-point value from a source by a scalar single-precision floating-point value from a second source, and adds the result to a scalar single-precision floating-point value from a third source, storing the result in a destination. The operation is performed as: $dest = (src1 \times src3) + src2$.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r32, m32 | r32 |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the FMA3 feature set to be supported by the processor.

To avoid precision loss or incorrect results, the user SHALL ensure that the floating-point control word is correctly configured, as this instruction is subject to rounding and exception flags. The instruction may trigger #D, #O, #U, or #P depending on the input values and the rounding mode. Invalid operations (#I) occur if any operand is a NaN or if an infinity is multiplied by zero.