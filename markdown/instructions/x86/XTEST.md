> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XTEST

XTEST performs a logical AND operation between the source operand and the destination operand, updating the ZF, SF, and PF flags based on the result, but does not store the result.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

XTEST is available only in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

When using an immediate value as the source, the immediate MUST be a signed i32, which is then sign-extended to the size of the destination register (r64) before the operation. Failure to account for sign-extension when using large immediate values may lead to incorrect flag results.