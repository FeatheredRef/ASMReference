> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KSHIFTRB

Shifts the bits of the source operand to the right by the count specified in the immediate operand. The shifted-out bits are filled with zeros.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| rN | rN |
| mN | rN |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

The immediate operand MUST be a constant value between 0 and 63. Using an immediate value outside of this range will result in an invalid encoding. Since this is a constant shift, the shift count is encoded directly into the instruction rather than being read from a register, which avoids the masking of the count to 5 or 6 bits typical of other shift instructions.