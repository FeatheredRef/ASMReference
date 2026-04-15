> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUNPCKHQDQ

Unpacks the high quadwords from two 128-bit XMM registers into a single 256-bit YMM register. Specifically, it selects the high 64 bits of the first source operand and the high 64 bits of the second source operand, then places them into the lower 128 bits of the destination register, while the upper 128 bits of the destination register are zeroed.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | ymm |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It REQUIRES the processor to support the AVX instruction set.

To avoid undefined behavior or illegal instruction exceptions, ensure that the destination register is a YMM register, as the operation expands 128-bit inputs into a 256-bit output. Mixing AVX instructions with legacy SSE instructions without using `VZEROUPPER` MAY result in performance penalties due to AVX-SSE transitions.