> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSRAVW

Shift right arithmetic each 32-bit element of a packed 256-bit vector by the count specified in the immediate or the source register. The sign bit of each 32-bit element is replicated into the most significant bits.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m32 | reg |
| imm | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX2 instruction set extension to be supported by the processor.

The shift count is masked to 5 bits (0-31) for 32-bit elements; any bits beyond the 5th bit in the immediate or register are ignored. Failure to mask the shift count manually before execution may result in unintended shift amounts if the value exceeds 31.