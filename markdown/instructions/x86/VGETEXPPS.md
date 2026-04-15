> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETEXPPS

Computes the exponent of four packed single-precision floating-point values. For each packed element, the instruction extracts the exponent and returns it as a packed single-precision floating-point value.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 foundation and requires the CPUID.01H.EAX bit 16 (AVX512F) to be set. It operates in 64-bit mode and compatibility mode.

The instruction behavior is subject to the rounding mode specified in the MXCSR register. If the input value is a signaling NaN, a #I exception shall be generated. If the result cannot be represented within the precision of f32, a #P exception may be generated. Use of this instruction on non-AVX-512 capable hardware will result in an `#UD` exception.