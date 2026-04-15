> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMI2B

This instruction selects bytes from two source operands based on an immediate value. For each byte position in the destination, the immediate specifies whether the byte is taken from the first source operand or the second source operand, and at which index within that operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m16 | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX-512 VBMI (Vector Byte Manipulation Instructions) extension.

To avoid undefined behavior or illegal instruction exceptions, the immediate value SHALL be encoded correctly according to the permutation map; indices exceeding the operand width are not supported. The destination register SHALL NOT be used as a source operand if the operation requires strict non-destructive behavior, although VPERMI2B generally supports three-operand syntax.