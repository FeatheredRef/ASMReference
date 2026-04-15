> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SAVEPREVSSP

Saves the current value of the RSP register into the specified destination operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| RSP | m64 |
| RSP | r64 |
| #I | imm |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is NOT available in compatibility mode or 32-bit mode.

When using a memory destination (m64), the instruction SHALL perform a store operation. If the memory operand is located in a non-writable page, a page-fault exception SHALL be generated. Ensure the destination is 8-byte aligned to avoid performance degradation or alignment check exceptions if Alignment Check (AC) flag is set in EFLAGS.