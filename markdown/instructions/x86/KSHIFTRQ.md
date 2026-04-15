> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KSHIFTRQ

Shifts the quadword value in the destination operand to the right by the count specified in the source operand. The shift is logical, meaning zeros are shifted into the most significant bits.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is not supported in compatibility mode.

The shift count is masked to 63 bits (count AND 63). If the shift count is 64 or greater, the result is not defined by a simple linear shift of the bits, but by the modulo 64 masking of the shift amount. This behavior is critical to avoid unexpected results when using a register as the shift count.