> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MPSADBW

Computes the sum of absolute differences between 8-bit signed integers. It treats the source and destination operands as vectors of 8-bit signed integers. For each 32-bit block, it calculates the absolute difference between corresponding bytes of the two operands for four different offsets, sums these differences, and stores the resulting four unsigned 32-bit integers in the destination.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |
| #I | imm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The destination register is overwritten by the result; therefore, if the source xmm register is also used as the destination, the original data is lost. The operation ignores the upper 64 bits of the xmm registers if operating on 128-bit vectors. Alignment of the m16 operand must follow the memory alignment rules for the specific processor architecture to avoid performance penalties or alignment faults.