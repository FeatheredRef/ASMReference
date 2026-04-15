> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KANDNQ

Performs a bitwise AND operation between a qword immediate and a qword destination, storing the result in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r64 |
| imm | m8 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It SHALL NOT be used in compatibility mode.

The immediate operand MUST be a qword. Ensure the destination memory operand is aligned to avoid performance penalties or faults depending on the processor's alignment check settings.