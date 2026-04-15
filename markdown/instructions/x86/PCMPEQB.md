> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPEQB

Compares each byte of the first operand with each byte of the second operand. For each pair of equal bytes, the corresponding byte of the destination operand is set to all ones (0xFF); otherwise, it is set to all zeros (0x00).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in SSE2 or a later architectural state. It requires the XMM register set.

The destination register is overwritten; therefore, the original value of the destination register SHALL be preserved in a separate register if it is required for subsequent operations. The instruction operates on 128-bit vectors; any bits beyond the 128-bit limit in the registers are unaffected.