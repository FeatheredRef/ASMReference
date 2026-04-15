> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFPCLASSPS

Classifies the scalar single-precision floating-point value in the source operand according to the specified classification. The result is returned as an unsigned 32-bit integer in the destination operand, where the value indicates whether the input is quiet NaN, signaling NaN, infinity, denormal, zero, or a normal number.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in SSE mode. It requires support for the SSE3 instruction set extension.

The destination register is overwritten with a 32-bit integer representing the class of the input `f32`. Because the result is stored in an XMM register, the upper 32 bits of the destination register's lower 64-bit lane are typically zeroed or ignored depending on the specific microarchitecture implementation, but the logical result is a `u32`. Use of this instruction on non-canonical floating-point values will not trigger floating-point exceptions; it classifies the bit pattern regardless of whether the value is a signaling or quiet NaN.