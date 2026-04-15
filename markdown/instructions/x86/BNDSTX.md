> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BNDSTX

Stores the lower 32 bits of the bound register in the destination operand and the upper 32 bits of the bound register in the subsequent memory location.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m4 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The destination operand MUST be a memory location. Since the instruction writes 8 bytes total (two dwords), it SHALL ensure that the destination memory address is aligned to avoid potential performance penalties or faults depending on the alignment check settings. This instruction specifically interacts with the MPX (Memory Protection Extensions) bound registers; if MPX is not supported by the processor or disabled in the BIOS/OS, this instruction SHALL trigger an invalid opcode exception.