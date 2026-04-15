> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTI64x2

Inserts two 64-bit quadwords from a source into a destination vector register at the index specified by the immediate.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The immediate value MUST be 0, 1, 2, or 3. If the immediate value is outside this range, the instruction is invalid.

The destination register is partially overwritten. Bits not targeted by the insertion are preserved. Using an incorrect immediate value for the intended vector lane will result in data corruption of adjacent elements.