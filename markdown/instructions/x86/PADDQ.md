> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PADDQ

Adds two 64-bit signed integers from the source operands and stores the resulting 64-bit signed integer in the destination.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE2 instruction set.

The operation is performed on the lowest 64 bits of the XMM registers. The upper bits of the destination XMM register are not modified. When performing additions, if an overflow occurs, the result is wrapped according to two's complement arithmetic; no exceptions (#O) are generated.