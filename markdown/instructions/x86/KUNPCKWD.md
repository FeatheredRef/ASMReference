> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KUNPCKWD

Unpacks a word from each of two 64-bit source operands into a double word in the destination operand. Specifically, it extracts the word from the low-order 16 bits of the first source and the word from the low-order 16 bits of the second source, then concatenates them to form a 32-bit double word.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE3 instruction set extension to be supported by the hardware.

The destination register is written to in the lower 32 bits of the XMM register; the upper bits of the destination XMM register remain unchanged. Failure to ensure the source XMM registers are properly loaded with 64-bit values may result in unexpected data being unpacked if the high bits are not cleared or initialized.