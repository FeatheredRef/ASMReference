> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB213PD

Performs a fused multiply-add or multiply-subtract operation on packed double-precision floating-point values. Based on the immediate operand, it computes either `destination = (source1 * source2) + destination` or `destination = (source1 * source2) - destination`. The "213" notation specifies that the destination is the first operand, the second source is the second operand, and the first source is the third operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m64/m128/m256/m512 |
| m64/m128/m256/m512 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX and FMA3 instruction set extensions.

The instruction utilizes the MXCSR register to control rounding and exception handling. Improper configuration of the rounding mode SHALL result in precision errors denoted by #P. If the result cannot be represented within the destination's precision, the instruction MAY trigger #O or #U. Denormalized inputs SHALL trigger #D unless the Floating-Point Assist (FP Assist) or Flush-to-Zero (FTZ) / Denormals-Are-Zero (DAZ) bits are set in MXCSR.