> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMAXSB

Compares the signed 8-bit integer values of the source and destination operands and stores the maximum of the two in the destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r8 | r8 |
| m1 | r8 |
| r8 | m1 |
| #I | m1 |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It is not supported in 32-bit protected mode.

The instruction operates on signed integers; using it for unsigned comparisons will yield incorrect results. Ensure that the data in the r8 or m1 operands is treated as a signed i8 to avoid logical errors in maximum value determination.