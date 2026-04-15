> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KORW

Performs a bitwise logical exclusive OR operation between a word-sized source operand and a word-sized destination operand, storing the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm | r16 |
| reg | r16 |
| imm | m2 |
| reg | m2 |

Support LOCK

This instruction is available in 64-bit mode and compatibility mode. In 64-bit mode, the operand size must be set to 16 bits via a prefix to target word-sized registers or memory.

The destination operand MUST NOT be the same as the source operand when using a memory reference if an atomic operation is required via the LOCK prefix, although the hardware typically handles this. If the destination is a memory operand, the operation is performed on a word-aligned address to avoid performance penalties or alignment checks.

The EFLAGS register is affected: the CF and OF flags SHALL be cleared, and the SF, ZF, and PF flags SHALL be updated based on the result of the operation.