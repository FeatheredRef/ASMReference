> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KSHIFTLD

This instruction shifts the value of a source operand to the left by a count specified in a register or immediate, then loads the result into a destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode.

The shift count is masked to five bits (0-31) for 32-bit operands and six bits (0-63) for 64-bit operands. Failure to mask the count manually before execution may result in unintended shift distances.