> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVAPS

Moves aligned packed single-precision floating-point values from a source to a destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m16 | reg |
| reg | m16 |

DO NOT support LOCK

The instruction SHALL be used only with XMM registers or memory addresses aligned on a 16-byte boundary. If the memory operand is not aligned to 16 bytes, a general-protection exception (#GP) SHALL be generated. This instruction is supported in both 64-bit mode and compatibility mode.

To avoid a #GP exception, ensure that any m16 operand is aligned to a 16-byte boundary. If alignment cannot be guaranteed, MOVUPS SHALL be used instead.