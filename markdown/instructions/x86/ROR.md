> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ROR

Rotates the bits of the destination operand to the right by the count specified by the source operand. Bits rotated out of the least significant position are wrapped around to the most significant position.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. When operating on r64, the rotation count is masked to 6 bits (modulo 64). When operating on r32, the rotation count is masked to 5 bits (modulo 32). When operating on r16, the rotation count is masked to 4 bits (modulo 16).

The Carry Flag (CF) is updated: the least significant bit of the operand before rotation is shifted into CF. The Overflow Flag (OF) is set if the rotation count is not 0. The Sign Flag (SF), Zero Flag (ZF), and Parity Flag (PF) are affected based on the resulting value.

If the rotation count is 0, the destination operand remains unchanged and CF is not affected. Ensure the rotation count is correctly masked by the hardware to avoid unexpected behavior when using a register as the count source.