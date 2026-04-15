> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUNPCKHBW

Unpacks high bytes of words. For each word-sized element in the source, the instruction extracts the high byte and replicates it into two consecutive bytes in the destination.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in 32-bit and 64-bit modes. It requires SSE2 support.

The instruction operates on XMM registers; using this instruction on non-XMM registers SHALL result in an invalid operation. The destination register is overwritten, so the source register MUST be preserved in a separate register if the original data is required.