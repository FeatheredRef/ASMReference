> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GF2P8AFFINEQB

Performs an affine transformation on each 8-bit element of the source operand using a specified matrix and vector, then stores the result in the destination operand. The operation is performed in GF(2^8) using the polynomial x^8 + x^4 + x^3 + x + 1.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available only when the processor supports the GLU (Galois Field and Carry-less Multiplication) instruction set extension. It requires the processor to be in 64-bit mode or compatibility mode.

The instruction operates on 128-bit XMM registers. To avoid undefined behavior or general protection faults, the memory operand MUST be aligned to the requirements of the addressing mode used. Incorrect alignment on certain platforms may trigger alignment check exceptions.