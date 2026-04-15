> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB213PH

Performs a fused multiply-add or multiply-subtract operation on half-precision floating-point values. Depending on the immediate operand, it calculates $f16 = (f16_{src1} \times f16_{src2}) + f16_{src3}$ or $f16 = (f16_{src1} \times f16_{src2}) - f16_{src3}$ for each corresponding element in the vectors.

The following table covers the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| mN | reg |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or 32-bit mode. It requires AVX-512 support (specifically AVX-512FP16). It is NOT available in compatibility mode.

The operation is performed according to IEEE 754-2008 standards. Intermediate calculations are performed with infinite precision before rounding to the final $f16$ result.

To avoid precision loss or unexpected exceptions, ensure the MXCSR register is configured for the desired rounding mode. The instruction may trigger the following exceptions based on the result: #I, #D, #O, #U, and #P. If the destination register is also used as a source, the original value is overwritten only after the operation is completed.