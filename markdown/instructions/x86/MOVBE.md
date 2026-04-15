> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVBE

MOVBE moves a value from a source to a destination while reversing the byte order of the data.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | mN |
| mN | reg |
| #I | imm |

DO NOT support LOCK

The instruction is only available in 64-bit mode or compatibility mode when the processor supports the MOVBE feature. It is not available in 32-bit mode.

The operand size must be 1, 2, 4, or 8 bytes. The instruction performs a big-endian to little-endian (or vice versa) conversion; users MUST ensure the operand size matches the target data width to avoid incorrect byte reversal or memory corruption. This instruction is intended to replace the sequence of `MOV` followed by `BSWAP`.