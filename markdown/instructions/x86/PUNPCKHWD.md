> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUNPCKHWD

Unpacks words from a 128-bit XMM register to doublewords. For each word in the source, the instruction creates a doubleword in the destination by copying the word and filling the upper 16 bits of the resulting doubleword with zeros. The operation is performed on the upper 64 bits of the source register to populate the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode and 32-bit mode. It requires the SSE3 extension to be supported by the processor.

The destination register is overwritten; therefore, the source register MUST be used as the destination if the original source data is required for subsequent operations. Since the instruction only processes the upper 64 bits of the source register to produce the 128-bit result, the lower 64 bits of the source register are ignored.