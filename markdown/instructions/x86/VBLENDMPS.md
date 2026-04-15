> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VBLENDMPS

Blends two packed single-precision floating-point vectors based on a mask specified by an immediate value. For each element, if the corresponding bit in the mask is 0, the element from the first source operand is selected; if the bit is 1, the element from the second source operand is selected.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm, xmm reg, xmm reg / m128 | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The immediate operand SHALL be an 8-bit value, but only the lowest 2 bits are used for `VBLENDMPS` as it operates on 4 single-precision elements (though the instruction specifically uses the immediate to select between the two source registers per element). If the destination register is also one of the source registers, the operation is performed in-place. Memory operands MUST be 128-bit aligned to avoid potential performance penalties or faults depending on the alignment check settings.