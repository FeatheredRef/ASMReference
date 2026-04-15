> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCODEKEY128

ENCODEKEY128 encodes a 128-bit key used for the AES-NI instruction set. It takes a 128-bit value from a source and transforms it into an expanded key stored in the destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the AES-NI feature is enabled in the processor. It is supported in both 32-bit and 64-bit modes.

The instruction requires the destination register to be an XMM register. Using a general-purpose register or a memory operand as a destination SHALL result in an invalid opcode exception. Failure to properly initialize the source XMM register with a valid 128-bit key will result in the generation of an incorrect expanded key without triggering a processor exception.