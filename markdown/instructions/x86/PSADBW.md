> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSADBW

Computes the absolute difference between unsigned 8-bit integers in a source operand and the unsigned 8-bit integer specified in the low byte of a destination operand. It sums these absolute differences for each group of 8 bytes, producing eight u32 results stored in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The destination register is both a source and the destination. Because the low byte of the destination xmm register is used as the subtraction operand, its initial value is overwritten by the result. If the destination is also the source, the original value in the low byte must be preserved in a separate register if it is needed after the operation. All results are truncated to u32; overflow beyond 32 bits per group is not handled by the instruction.