> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XCHG

Interchanges the contents of a destination operand and a source operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| mN | reg |
| imm | #I |
| mN | mN |

Support LOCK

XCHG is only available in 64-bit mode, 32-bit mode, and compatibility mode. If the destination is a memory operand, the instruction is implicitly locked, regardless of whether the LOCK prefix is explicitly used.

When performing an exchange between a register and memory, the operand size MUST match the register size (e.g., r64 with m8 is invalid). Use of the LOCK prefix on a register-to-register XCHG is ignored.

In 64-bit mode, if the memory operand is accessed via a RIP-relative address, the size of the displacement is a 32-bit signed integer. Failure to align memory operands to their natural boundary MAY result in performance degradation or atomicity failure across cache lines.