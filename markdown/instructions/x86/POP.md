> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# POP

The instruction pops the value from the top of the stack (pointed to by RSP) into the destination operand and subsequently increments the stack pointer by the size of the popped operand.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| m8 | r8 |
| m16 | r16 |
| m32 | r32 |
| m64 | r64 |

DO NOT support LOCK

The instruction SHALL operate according to the operand size determined by the current execution mode (64-bit mode or compatibility mode). In 64-bit mode, if a 32-bit register is used as the destination, the upper 32 bits of the destination register SHALL be zero-extended.

When popping a 64-bit value, the RSP is incremented by 8 bytes. If the operation results in a stack wrap-around, it MAY trigger a general-protection exception (#GP) depending on the canonical address rules of the x86-64 architecture.

Users SHALL ensure that the stack pointer (RSP) is properly aligned before executing POP to avoid potential alignment check exceptions if the Alignment Check (AC) flag in CR0 is enabled. Failure to match the POP operand size with the corresponding PUSH operand size SHALL result in stack pointer misalignment, leading to corrupted data retrieval or application crashes.