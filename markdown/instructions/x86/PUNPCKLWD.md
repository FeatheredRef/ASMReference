> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUNPCKLWD

Unpacks low 32-bit elements from a 128-bit XMM source to a 128-bit XMM destination. For each 64-bit quadword in the source, the instruction interleaves the low 32-bit double word with zeros, placing the double word in the low 32 bits of the destination quadword and zeroing the high 32 bits.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m16 | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE3 extension set to be supported by the processor.

The destination register is overwritten by the result; therefore, if the source is a register and the destination is the same register, the original value is lost. Since this instruction performs zero-extension of 32-bit elements into 64-bit slots, any existing data in the upper 32 bits of the target double words in the destination register will be cleared.