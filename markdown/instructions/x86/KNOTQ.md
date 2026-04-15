> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KNOTQ

Performs a bitwise logical NOT operation on the destination operand. Each bit in the destination is inverted (0 becomes 1, and 1 becomes 0).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| #I | r64 |
| #I | m8 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode.

The destination operand MUST be a 64-bit register or a 64-bit memory location. Attempting to use a smaller register size will result in an invalid operation. Since this instruction performs a unary operation, there is no source operand; providing an immediate or an additional register is NOT permitted.