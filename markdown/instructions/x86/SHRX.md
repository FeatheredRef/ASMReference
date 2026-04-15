> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHRX

Shifts the source operand to the right by the count specified in the second operand. Unlike SHR, SHRX is a non-destructive shift, meaning the destination operand is separate from the source operand. The vacated bit positions are filled with zeros.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m64 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

The shift count is determined by the low-order 6 bits of the second operand (the register containing the count), meaning the actual shift amount is u6 (0-63). Any bits beyond the 6th bit in the count register are ignored. Using a register for the count is REQUIRED; an immediate value is NOT supported.