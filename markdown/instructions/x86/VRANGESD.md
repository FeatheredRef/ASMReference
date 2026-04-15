> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRANGESD

This instruction compares a value in a register against a range specified by a memory operand (the range descriptor) and updates a mask register based on whether the value falls within the specified range.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m64 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode or 32-bit mode.

To avoid undefined behavior or general protection faults, the memory operand MUST be properly aligned to the requirements of the range descriptor structure. Failure to ensure that the source register contains a value compatible with the expected data type of the range descriptor MAY result in incorrect mask generation.