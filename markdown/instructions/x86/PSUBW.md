> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSUBW

Subtracts two packed signed 16-bit integers from a XMM register and stores the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m2 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates on the lower 128 bits of the XMM registers.

The instruction performs a signed subtraction. To avoid unexpected behavior, ensure that the input values do not exceed the range of a signed 16-bit integer (i16), as the instruction does not perform saturation; it will wrap around on overflow.