> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTSD2SS

Converts a scalar double-precision floating-point value to a scalar single-precision floating-point value. The conversion is performed according to the rounding control in the MXCSR register.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m64 | xmm |
| xmm | xmm |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It requires the SSE3 instruction set extension.

The destination register is overwritten by the result; therefore, the source and destination registers MAY be the same. If the conversion results in an overflow, the destination is set to the signed infinity of the corresponding sign. If the result is an underflow, it is converted to a denormalized number or zero. The following floating-point exceptions MAY be raised: #O, #U, #D, and #P.