> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMP

Subtracts the source operand from the destination operand and updates the flags according to the result, but does not store the result in the destination.

The following table covers what the source and destinations can be:

| destination(s) | source |
| :--- | :--- |
| rN | rN |
| rN | imm |
| mN | rN |
| mN | imm |

DO NOT support LOCK

The instruction is supported in 64-bit mode, 32-bit mode, and 16-bit mode. The operand size is determined by the operand-size override prefix or the default for the current operating mode.

The destination operand SHALL NOT be an immediate value. Because the result of the subtraction is discarded, the destination register or memory location remains unchanged. This instruction is used exclusively to set the EFLAGS register (specifically CF, ZF, SF, OF, PF) to facilitate conditional branching.