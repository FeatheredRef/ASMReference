> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMAXUB

Compares two unsigned 8-bit integers and stores the maximum value in the destination.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension to be supported by the processor.

The instruction operates on packed data; therefore, the number of elements processed depends on the size of the XMM registers (128-bit). Failure to align memory operands (if applicable via related load/store instructions) may result in performance degradation or faults depending on the alignment check flag.