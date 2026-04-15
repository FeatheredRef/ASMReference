> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUNPCKLBW

Unpacks bytes from two XMM operand registers into words. It selects the low-order byte from each of the two source operands and interleaves them into the destination register. Specifically, it takes the first byte of the first operand and the first byte of the second operand to form the first word, then the second byte of the first operand and the second byte of the second operand to form the second word, and so on, for a total of 8 words (128 bits).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It requires SSE3 support to be enabled in the processor.

The instruction operates exclusively on XMM registers; attempting to use memory operands directly in the instruction encoding will result in an invalid operation. Ensure that the source registers are properly loaded via `MOV` or `MOVAPS` before execution to avoid alignment exceptions if memory is involved in the preceding data movement.