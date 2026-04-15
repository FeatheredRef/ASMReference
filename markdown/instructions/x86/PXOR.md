> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PXOR

Performs a bitwise logical exclusive OR operation between two operands. The result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |
| imm | #I |
| mN | mN | #I |

Support LOCK

PXOR is specifically designed for XMM registers (128-bit). It operates in the SIMD domain. In x86-64, this instruction is available in both 64-bit mode and compatibility mode.

The destination operand MUST be an XMM register or a memory location capable of storing 128 bits. Using the instruction with non-XMM registers is invalid. When the destination is a memory operand, the instruction is subject to alignment constraints; unaligned memory accesses MAY cause performance penalties or faults depending on the processor's alignment check (AC) flag in CR0.