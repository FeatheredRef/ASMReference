> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHR

The `SHR` instruction shifts the destination operand to the right by the count specified in the source operand. Vacant bit positions are filled with zeros, and the last bit shifted out is stored in the CF flag.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | mN |
| imm | mN |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. When operating in 64-bit mode, if the destination is an r64 register, the shift count is masked to 6 bits (0-63).

If the shift count is greater than or equal to the operand size (e.g., shifting an r64 register by 64 or more), the result is undefined. To avoid unpredictable behavior, the shift count MUST be less than the bit-width of the destination operand.