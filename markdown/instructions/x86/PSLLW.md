> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSLLW

Shifts each 16-bit packed signed integer element of the destination operand to the left by the count specified in the source operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode.

The shift count is masked to 15 bits; if the source register contains a value greater than 15, only the lower 4 bits (0-15) are used. To avoid unexpected behavior, the shift count should be constrained to a value between 0 and 15.