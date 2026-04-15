> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BZHI

BZHI zeros the most significant bits of the destination operand above the index specified by the source operand. The index represents the number of bits to be retained, starting from the least significant bit.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m8 |
| reg | m16 |
| reg | m32 |
| reg | m64 |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

If the index provided in the source register is greater than or equal to the operand size (e.g., $\ge 64$ for a qword), the destination operand SHALL remain unchanged.