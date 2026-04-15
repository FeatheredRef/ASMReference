> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFCMADDCSH

Computes the fused multiply-add of two floating-point values and adds the result to a third value, using the specified precision (single or double) and rounding mode. Specifically, it performs the operation $a = (b \times c) + a$ where the operands are the high-order elements of the vectors.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm reg | zmm/ymm/xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 Foundation (F) instruction set. Execution in compatibility mode is NOT supported.

The instruction operates on the most significant elements of the provided registers; if the register sizes are mismatched, the operation may result in undefined behavior or trigger a General Protection exception (#GP). Ensure that the rounding mode specified in the immediate byte is correctly aligned with the intended precision to avoid #P or #O. Mixing different vector lengths (e.g., combining xmm with zmm) is NOT permitted for this specific operation.