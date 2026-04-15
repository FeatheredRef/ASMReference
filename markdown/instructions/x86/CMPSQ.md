> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPSQ

Compares the qword value in RSI with the qword value in RDI by subtracting the value in RSI from the value in RDI. The result of the subtraction is not stored; instead, the EFLAGS register is updated based on the result. If the Direction Flag (DF) is cleared, both RSI and RDI are incremented by 8; if DF is set, both are decremented by 8.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | m8 |
| reg | reg |
| #I | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It requires the use of the RSI and RDI registers specifically for the source and destination addresses; these cannot be replaced by other general-purpose registers.

To avoid unexpected behavior during string operations, the state of the DF flag MUST be explicitly managed using CLD (Clear Direction Flag) or STD (Set Direction Flag) before execution, as the pointer increment/decrement direction depends entirely on this flag. Additionally, ensure that the memory regions pointed to by RSI and RDI are properly aligned to avoid performance penalties or faults in specific environments.