> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADDSUBPS

Adds or subtracts packed single-precision floating-point values from the source to the destination. The operation is determined by the sign bit of a control register or a specific immediate/prefix depending on the implementation, performing `destination = destination ± source` for each corresponding f32 element in the XMM registers.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction requires the SSE3 instruction set extension. It SHALL NOT be used in environments where SSE3 is not supported. It operates on XMM registers and is available in both 64-bit mode and compatibility mode.

Alignment of the m128 operand SHALL be 16-byte aligned to avoid general protection faults unless the processor supports unaligned memory access. Failure to ensure alignment may result in performance degradation or application crashes.

The instruction may trigger the following floating-point exceptions:
- #D: If any operand is a denormalized value.
- #O: If the result exceeds the maximum representable f32 value.
- #U: If the result is smaller than the minimum representable f32 value.
- #P: If the result requires rounding.