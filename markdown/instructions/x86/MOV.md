> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOV

Copies the value from the source operand to the destination operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| mN | reg |
| reg | mN |
| imm | mN |

DO NOT support LOCK

The instruction SHALL NOT allow both operands to be memory regions; one operand MUST be a register. In 64-bit mode, if an immediate is used as a source for a register destination, the immediate MUST be a sign-extended 32-bit value unless the `MOVABS` opcode is utilized for a 64-bit immediate.

When moving data into a 32-bit register (r32) in 64-bit mode, the upper 32 bits of the destination 64-bit register (r64) SHALL be zeroed. To avoid unintended data loss in the upper bits of a register, the programmer MUST use the specific register size corresponding to the intended data width. Memory accesses MUST be aligned to the operand size to avoid performance penalties or exceptions on certain processor configurations.