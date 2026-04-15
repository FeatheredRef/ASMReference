> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PHADDW

Adds two packed signed 16-bit integers from two source operands and stores the resulting sums in the destination. For each pair of 16-bit elements, the result is the sum of the corresponding elements from the sources.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m2 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode when SSE3 is supported. It operates on XMM registers; it is NOT available in compatibility mode for operands exceeding the size of the current execution mode's register width if the specific SSE3 extension is absent.

The instruction performs signed addition. Since it does not utilize a saturation mechanism, integer overflow will result in wrap-around behavior according to two's complement arithmetic. Users MUST ensure that the input values do not exceed the range of i16 to avoid unintended wrap-around results.