> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTDQ2PS

Converts the packed signed doubleword integers (i32) from a source operand to packed single-precision floating-point values (f32) and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction requires SSE4.1 support. It operates on XMM registers and is available in both 32-bit and 64-bit modes.

The instruction converts two i32 values into two f32 values. If the source is an xmm register, only the lower 64 bits of the source are used. The upper 64 bits of the destination xmm register are zeroed. Precision exceptions (#P) may be raised if the integer value cannot be represented exactly as a single-precision floating-point number.