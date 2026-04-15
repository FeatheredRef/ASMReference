> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PADDUSB

Adds unsigned 8-bit integers from a source to a destination, saturating the result to 255 if the sum exceeds the maximum value of an unsigned byte.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only when SSE3 is supported. It operates on packed data within XMM registers; it is NOT supported for general-purpose registers or direct memory-to-memory operations.

The instruction REQUIRES the destination operand to be an XMM register. To avoid unexpected behavior, ensure that the data is properly aligned to 16 bytes when loading from memory into XMM registers before using PADDUSB, as unaligned access may result in performance degradation or faults depending on the specific memory operand instruction used.