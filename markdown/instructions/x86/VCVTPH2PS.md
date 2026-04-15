> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2PS

Converts packed half-precision floating-point values to packed single-precision floating-point values.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm-reg | xmm-reg |
| m128 | xmm-reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode. It requires the SSE3 and AVX instruction sets to be supported by the processor.

The instruction converts two 16-bit floating-point values (f16) from the source to two 32-bit floating-point values (f32) in the destination. If the source is an xmm register, only the lower 32 bits of the destination register are updated in the non-AVX version, or the destination is overwritten based on the specific VEX encoding. 

When converting from half-precision to single-precision, no precision loss occurs; however, if the source value is a signaling NaN, a #P exception may be generated depending on the MXCSR register settings. Users MUST ensure that the destination register is properly initialized if utilizing the instruction in a way that does not overwrite the entire register.