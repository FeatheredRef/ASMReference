> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMINUB

Computes the minimum of the unsigned 8-bit integers in the source and destination operands and stores the result in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode and compatibility mode. It requires the SSE4.1 instruction set extension.

The operation is performed on each corresponding pair of unsigned 8-bit integers in the XMM registers. Since it operates on packed bytes, if the registers are not fully utilized, the remaining elements are processed according to the register width. Ensure that the XMM registers are properly aligned to avoid performance penalties, although it is not a requirement for correctness.