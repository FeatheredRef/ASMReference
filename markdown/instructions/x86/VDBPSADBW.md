> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VDBPSADBW

Computes the sum of absolute differences between the unsigned 8-bit integers in the source operand and the unsigned 8-bit integers in the destination operand. Specifically, it subtracts the source byte from the destination byte, takes the absolute value of the result, and accumulates the sums into 64-bit signed integers within the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m16 | xmm |
| reg | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The destination register is overwritten by the result of the operation; therefore, the original values in the destination xmm register are lost. To avoid data loss, the destination register MUST be backed up if its initial content is required for subsequent operations.