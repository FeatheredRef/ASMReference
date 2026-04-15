> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DPPS

Computes the dot product of two packed 32-bit floating-point vectors. The instruction multiplies corresponding elements of the source operands, sums the products, and stores the result in the destination. The result is combined with the destination's existing value according to the specific opcode variant.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the SSE4.1 extension set to be supported by the processor.

The destination register is used as both a source and a destination. If the destination register is not the same as the first source operand, the behavior depends on the specific encoding variant (e.g., whether the result is added to the existing destination value or replaces it). To avoid unexpected results, ensure the destination register is initialized or the correct opcode variant is selected for the intended accumulation behavior. Failure to manage the destination register state may lead to incorrect floating-point sums.