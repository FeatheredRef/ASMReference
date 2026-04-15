> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BT

Copies the value of a specified bit from the source operand to the Carry Flag (CF). The bit selected is determined by the index specified in the source operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8, r8 | CF |
| m16, r16 | CF |
| m32, r32 | CF |
| m64, r64 | CF |

DO NOT support LOCK

The instruction is available in both 64-bit mode and compatibility mode. When operating in 64-bit mode, if the source operand is a 64-bit register or memory location, the index must be within the range of 0 to 63.

The bit index is specified by the register used as the second operand. If the index exceeds the width of the source operand (e.g., index $\ge 64$ for a qword), the behavior is defined such that the Carry Flag is cleared. Users MUST ensure the index register contains a value within the valid bit range of the source operand to avoid unexpected CF results.