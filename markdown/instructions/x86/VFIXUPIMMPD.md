> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFIXUPIMMPD

Fixes up the immediate value provided as an operand and stores the resulting 64-bit double-precision floating-point value into the destination.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| imm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is not supported in compatibility mode.

The immediate operand is interpreted as a 32-bit integer and converted to a double-precision floating-point format. Failure to use this instruction in 64-bit mode SHALL result in an invalid opcode exception.