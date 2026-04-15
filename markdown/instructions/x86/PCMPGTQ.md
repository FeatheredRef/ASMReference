> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPGTQ

Compares two quadword signed integers. If the first operand is greater than the second operand, the destination is set to all ones (-1); otherwise, the destination is set to all zeros (0).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m8 | r64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension to be supported by the hardware.

The operands MUST be XMM registers; the notation r64 refers to a 64-bit element within an XMM register. Failure to use the correct register type will result in an invalid opcode exception. This instruction does not affect any EFLAGS register bits.