> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PINSRQ

Inserts 64 bits of data from the source operand into a destination XMM register at a byte offset specified by the immediate operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r64 | xmm |
| m8 | xmm |

DO NOT support LOCK

The instruction is only available in 64-bit mode. It requires the SSE4.1 instruction set extension.

The immediate operand MUST be a multiple of 8. If the immediate operand is not a multiple of 8, the instruction SHOULD trigger an invalid operand exception. The destination register MUST be an XMM register; accessing YMM or ZMM registers via this specific mnemonic is not supported.

The immediate operand MUST be within the range of 0 to 15 bytes to avoid accessing memory beyond the 128-bit boundary of the XMM register. Failure to adhere to this range SHALL result in an undefined operation or a general protection fault depending on the processor implementation.