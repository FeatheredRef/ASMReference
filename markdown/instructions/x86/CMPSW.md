> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPSW

Compares the word-sized value in the source operand with the word-sized value in the destination operand. The operation subtracts the source from the destination and sets the EFLAGS register based on the result, but the destination operand is not modified.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r16 | m2 |
| r16 | r16 |
| m2 | m2 |
| m2 | r16 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. In 64-bit mode, the instruction operates on 16-bit operands.

The instruction affects the ZF, SF, OF, PF, and CF flags. Because this is a comparison operation, the destination operand MUST NOT be modified. If used with a memory destination, the memory write-back is suppressed. User MUST ensure that the registers used as pointers are correctly aligned to 2-byte boundaries to avoid performance penalties or alignment faults.