> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSHRD

Shifts the unsigned doublewords in each 128-bit lane of a YMM register to the right by the count specified in the source operand. The bits shifted out are filled with zeros.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX set to be enabled in the processor.

The shift count is masked to 31 (5 bits) for doubleword elements; any bits beyond the 5th bit in the immediate or register source are ignored. Failure to mask the shift count manually is unnecessary as the hardware performs this operation automatically.