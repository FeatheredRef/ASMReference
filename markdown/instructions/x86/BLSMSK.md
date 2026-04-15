> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BLSMSK

Sets the destination operand to the mask of the lowest set bit (LSB) of the source operand. If the source operand is zero, the destination operand is set to zero.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| #I | mem |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is not supported in compatibility mode.

To avoid incorrect results, ensure the destination register is cleared or overwritten, as the instruction specifically modifies the register based on the bit position of the lowest set bit of the source; failure to account for the zero-case (where the source is 0) may lead to logic errors in bitmasking operations.