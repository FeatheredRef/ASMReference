> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSHRDV

Shifts the unsigned doublewords in the destination operand to the right by the count specified in the source operand. The shift is logical, meaning zeros are shifted into the most significant bits.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | xmm/ymm/zmm |
| imm | xmm/ymm/zmm |
| mN | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 foundation instructions set to be supported by the processor.

The shift count is masked to 5 bits (u5); any bits beyond the 5th bit in the source operand are ignored. Failure to mask the shift count manually before passing it as a register operand may result in unexpected shift distances due to this internal masking.