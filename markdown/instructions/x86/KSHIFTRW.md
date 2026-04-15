> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KSHIFTRW

Shifts the bits of a qword operand to the right by the count specified in a register or immediate. The operation is performed on a word-sized operand, where the value is shifted and the result is stored back.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| imm | reg |
| reg | reg |
| #I | mN |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It SHALL NOT be used in compatibility mode.

Ensure that the shift count provided in the register or immediate is masked to 5 bits (0-31) for qword operations to avoid undefined behavior, as per standard x86-64 shift logic. Incorrectly calculating the shift count may lead to unexpected results if the value exceeds the register width.