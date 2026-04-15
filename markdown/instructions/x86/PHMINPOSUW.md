> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PHMINPOSUW

PHMINPOSUW compares two unsigned word values and stores the index (position) of the minimum value into the destination operand. If the values are equal, the index of the second operand is stored.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, m2 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode.

To avoid incorrect results, ensure the destination register is treated as a 64-bit register, as the index returned is a 64-bit unsigned integer representing the position. Failure to account for the register size may lead to truncated index values.