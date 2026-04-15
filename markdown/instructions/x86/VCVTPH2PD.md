> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2PD

Converts packed 16-bit floating-point values (half precision) to packed 64-bit floating-point values (double precision).

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode. It is not supported in compatibility mode. The operation is subject to the rounding control and precision control bits defined in the MXCSR register.

The instruction requires the AVX instruction set. If executed on a processor that does not support AVX, it will trigger an invalid opcode exception. Because the destination is a 256-bit register (XMM/YMM) but the conversion expands 16-bit values to 64-bit values, the upper 64 bits of the destination register are zeroed if the source contains only two half-precision values. Use of the V-prefix prevents the use of the destination register as an implicit source, avoiding the partial register stall associated with legacy SSE instructions.

Expected floating-point exceptions include:
- #I: If the input is a signaling NaN.
- #P: If the converted result is inexact.