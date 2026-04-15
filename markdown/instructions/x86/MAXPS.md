> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MAXPS

Compares four packed single-precision floating-point values in a source operand with four packed single-precision floating-point values in a destination operand. For each pair, the instruction selects the larger of the two values and stores the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction requires the SSE extension to be supported by the processor. It operates on XMM registers and is available in both 32-bit and 64-bit modes.

If any of the input operands are NaN, the result is determined by the specific NaN handling rules of the SSE instruction set. The instruction shall not trigger #Z, #O, or #U exceptions; however, it may trigger #P if the result is rounded. All operations must be performed on aligned memory (16-byte boundary) when using m128 to avoid alignment faults.