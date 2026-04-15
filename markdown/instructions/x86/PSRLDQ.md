> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSRLDQ

Shifts the unsigned quadword value in the source operand to the right by the count specified in the low-order 6 bits of the `ecx` register. Zeroes are shifted into the most significant bits.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | r64 |
| r64 | r64 |

DO NOT support LOCK

The instruction SHALL only be used in 64-bit mode. In compatibility mode, this instruction is not supported and will trigger an invalid opcode exception.

The shift count is exclusively determined by the value in `ecx`. The instruction ignores the upper 58 bits of the `rcx` register. Users MUST ensure the shift count is loaded into `ecx` before execution to avoid unpredictable results stemming from stale data in the register.