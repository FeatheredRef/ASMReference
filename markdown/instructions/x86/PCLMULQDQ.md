> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCLMULQDQ

Performs a carry-less multiplication of two 64-bit operands. The instruction multiplies the 64-bit values in the source operands as if they were polynomials over the finite field GF(2), resulting in a 128-bit product.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is in 64-bit mode or compatibility mode. It requires the SSE4.2 and PCLMUL instruction sets to be enabled.

The instruction operates exclusively on XMM registers; memory operands are NOT supported for the source or destination. To process data from memory, the data MUST be loaded into xmm registers using `MOVDQA` or `MOVUPD` before execution. Incorrect alignment of memory loads preceding this instruction SHALL result in a general protection exception (#GP).