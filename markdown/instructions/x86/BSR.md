> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BSR

Scans the source operand for the first set bit (1) starting from the most significant bit (MSB) and writes the index of that bit into the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| imm | #I |

DO NOT support LOCK

BSR is supported in 64-bit mode, but it operates on the operand size specified by the prefix or the default for the current mode. In 64-bit mode, if a r64 operand is used, the instruction scans 64 bits; if a r32 operand is used, it scans 32 bits.

If the source operand contains no set bits (is zero), the destination register remains unmodified and the Zero Flag (ZF) is set to 1. If at least one bit is set, ZF is cleared to 0. Because the destination register is not updated when the source is zero, the software MUST check the ZF before relying on the value in the destination register to avoid using stale data.