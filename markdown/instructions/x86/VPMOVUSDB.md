> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVUSDB

Zero-extends each byte from the source operand to a doubleword in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX extension.

The destination register is overwritten. Because this instruction expands 8-bit elements to 32-bit elements, it SHALL only be used with the 128-bit version of the instruction when the destination is an xmm register to avoid out-of-bounds access or unintended register overlapping if used in a wider YMM context without proper alignment. Memory sources SHALL be aligned to the size of the accessed data to avoid performance penalties or faults.