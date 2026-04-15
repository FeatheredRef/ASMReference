> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BSF

Scans the source operand for the first set bit (bit set to 1), searching from the least-significant bit toward the most-significant bit. If a set bit is found, the index of that bit is stored in the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

BSF is supported in both 64-bit mode and compatibility mode. The instruction operates on the size of the destination register (r8, r16, r32, or r64), which determines the width of the source operand.

If the source operand contains no set bits, the destination register is undefined and the Zero Flag (ZF) is set to 1. If the source operand contains at least one set bit, ZF is cleared to 0. Users MUST check the ZF before relying on the value of the destination register to avoid using undefined data.