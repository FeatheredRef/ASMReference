> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRTSH

Computes the approximate reciprocal square root of a packed signed short floating-point value.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |
| m128/m256/m512 | xmm/ymm/zmm register |
| imm | #I |

DO NOT support LOCK

This instruction is only available when the AVX-512 foundation (F) is supported. It operates on packed signed short (16-bit) floating-point numbers.

The result of the approximation is an estimate; users SHALL use the `VRSQRTSH` instruction only for initial guesses in iterative refinement methods (such as Newton-Raphson) if higher precision is required. The precision of the result is subject to the current rounding mode in the MXCSR register, and may trigger a #P exception.