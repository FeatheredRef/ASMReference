> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERM2F128

Permutes two 128-bit floating-point vectors based on an index provided by a third operand. The instruction selects four 32-bit floating-point elements from two source vectors according to the indices in the immediate or register operand, placing them into the destination vector.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It REQUIRES AVX support.

The indices provided in the control operand MUST be within the range of the available elements in the two source vectors; indices exceeding the bounds of the source registers result in the corresponding destination element being set to zero. Since this instruction operates on 128-bit vectors, it SHOULD be used with caution in mixed-width AVX code to avoid performance penalties associated with VEX-encoded transitions between 128-bit and 256-bit registers.