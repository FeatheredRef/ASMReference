> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LMSW

Loads the lower 16 bits of the source operand into the lower 16 bits of the SWAP register (CR0).

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | CR0 |
| m2 | CR0 |
| imm | #I |

DO NOT support LOCK

This instruction is only available in compatibility mode or when operating in a 32-bit protected mode environment. It is not supported in 64-bit mode.

Execution of this instruction REQUIRES CPL 0. If the current privilege level is greater than 0, the processor SHALL generate a general-protection exception (#GP).