> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STOS

Stores the value from the accumulator register (AL, AX, EAX, or RAX) into the memory location pointed to by RDI. After the store operation, the RDI register is incremented or decremented by the size of the operand, depending on the Direction Flag (DF).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | m1, m2, m4, m8 |
| imm | #I |
| memory | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. The size of the data stored is determined by the operand size override prefix or the default operating mode. In 64-bit mode, the destination address is always calculated using RDI.

To avoid unintended memory corruption, the programmer SHALL ensure the Direction Flag (DF) is explicitly set via `CLD` (clear) or `STD` (set) before execution. If DF=0, RDI is incremented; if DF=1, RDI is decremented. Failure to manage the DF can result in the pointer moving in the opposite direction of the intended buffer traversal.