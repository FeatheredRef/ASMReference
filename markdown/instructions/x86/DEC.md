> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DEC

Subtracts 1 from the value of the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| Implicit | reg |
| Implicit | mN |

Support LOCK

The `DEC` instruction is available in both 64-bit mode and compatibility mode. When operating on a memory operand, the size of the operand is determined by the operand-size override prefix or the default size for the current mode (e.g., qword in 64-bit mode).

The `DEC` instruction does not affect the Carry Flag (CF). This differs from the `SUB` instruction, which does affect CF. Consequently, using `DEC` for loop counters or pointer arithmetic may lead to incorrect results if the program relies on the Carry Flag to detect underflow.