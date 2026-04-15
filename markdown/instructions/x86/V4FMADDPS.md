> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# V4FMADDPS

Computes the sum of a product of two floating-point values and a third floating-point value (a * b + c) for single-precision floating-point values using the YMM registers.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m128 |
| m128 | reg |
| m128 | m128 |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX support bit to be enabled in the CPUID. If the processor is in a state where AVX is not enabled, executing this instruction SHALL result in an invalid opcode exception.

To avoid performance degradation due to AVX-SSE transitions, the developer SHOULD ensure that VEX-encoded instructions are not mixed with legacy SSE instructions without the appropriate transition sequences. The instruction results MAY trigger floating-point exceptions including #D, #O, #U, and #P depending on the rounding mode and the input values. Failure to handle these exceptions SHALL result in the behavior defined by the MXCSR register.