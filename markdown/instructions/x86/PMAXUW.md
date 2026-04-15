> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMAXUW

Compares two unsigned word operands and stores the maximum value in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m2 | reg |
| reg | m2 |
| reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the SSE4.1 extension set to be supported by the processor.

The destination register must be a 128-bit XMM register. Operations on word-sized elements within the XMM registers are performed in parallel (SIMD). Failure to align memory operands to 2-byte boundaries may result in performance degradation or faults depending on the alignment check (AC) flag in the EFLAGS register.