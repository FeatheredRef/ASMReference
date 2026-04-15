> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VALIGND

Aligns a double word (dword) boundary for a destination memory address by adding a value to the source operand such that the resulting address is aligned to a 16-byte boundary.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | m4 |
| imm | m4 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination operand MUST be a 4-byte memory region (m4) used to store the original address before alignment. Failure to ensure the destination is a valid memory address will result in a general protection fault.