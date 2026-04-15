> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BNDMK

Marks the upper bound of a bound range by setting the upper bound of the specified bound register to the value of the source operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| m64 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode.

The destination register MUST be one of the bound registers (BND0-BND7). Attempting to use a general-purpose register that is not a bound register as the destination will result in an invalid opcode exception.

When using an immediate value, the value is sign-extended to 64 bits before being loaded into the bound register. Ensure that the target bound register is correctly associated with the pointer being checked to avoid incorrect bounds checking via BNDCL or BNDCU.