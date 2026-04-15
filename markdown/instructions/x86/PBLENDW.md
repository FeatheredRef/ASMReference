> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PBLENDW

Selects 16-bit words from two XMM source operands based on a mask provided in a third XMM operand. For each word, if the corresponding mask bit is 1, the word is taken from the first source operand; otherwise, it is taken from the second source operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor supports the SSE4.1 instruction set. It operates exclusively on XMM registers; memory operands are not supported.

To avoid undefined behavior or incorrect results, the mask operand MUST be treated as a bitmask where each bit corresponds to a 16-bit word. Ensure that the input registers are properly aligned to 128-bit boundaries to prevent performance degradation, although the instruction itself does not trigger alignment faults.