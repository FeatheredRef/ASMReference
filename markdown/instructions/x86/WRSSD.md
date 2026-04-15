> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WRSSD

Writes a 64-bit value from a register to a specified location in the Single Precision Floating-Point Control and Status Register (MXCSR) based on an index provided in a register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | m8 |
| #I | imm |
| #I | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The destination operand MUST be a memory location; however, it acts as a pointer to the MXCSR register via an index. The index provided in the source register MUST be within the valid range for the MXCSR register; otherwise, the behavior is undefined.

The instruction uses a specific addressing mechanism to target the MXCSR register. Ensure that the memory operand refers to a valid memory-mapped or architectural interface for the control register to avoid general protection faults.