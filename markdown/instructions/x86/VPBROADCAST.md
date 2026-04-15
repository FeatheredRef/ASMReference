> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBROADCAST

Copies a value from a source operand and replicates it across all elements of the destination vector register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX or AVX-512 instruction set extensions depending on the specific operand size and register width used.

The destination register is overwritten; therefore, the original contents of the destination register are lost. When using memory operands, the memory address SHALL be aligned to the size of the data being read to avoid potential performance penalties or alignment faults, depending on the processor's alignment check configuration. If the instruction is used with AVX-512 masks, the masking behavior SHALL be verified to ensure that only the intended elements of the destination register are updated.