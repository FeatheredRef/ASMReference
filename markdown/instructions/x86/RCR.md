> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RCR

Rotates the destination operand right one bit, including the Carry Flag (CF) in the rotation. The Carry Flag is shifted into the most significant bit of the destination, and the least significant bit of the destination is shifted into the Carry Flag.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| #I | reg |
| #I | mN |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand sizes. In 64-bit mode, if a 32-bit operand size is used, the upper 32 bits of the destination register are preserved.

The destination operand MUST be the same size as the operand size specified by the prefix or the default architecture mode. Because the instruction relies on the state of the Carry Flag, the CF MUST be correctly initialized if a specific bit sequence is required for the rotation. The instruction modifies the CF and the flags SF, ZF, and PF based on the resulting value of the destination.