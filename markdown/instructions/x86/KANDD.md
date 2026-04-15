> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KANDD

Performs a bitwise AND operation between a source operand and a destination operand, then performs a double-precision floating-point conversion on the result.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | m64 |
| imm | m64 |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

The destination register MUST be a XMM register to accommodate the resulting f64 value. Failure to target an XMM register will result in an invalid opcode exception. Ensure that the memory operand, if used, is 8-byte aligned to avoid alignment check exceptions or performance penalties.