> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHA1MSG1

This instruction performs a SHA-1 message schedule update. It computes the sum of three 32-bit words and rotates the result, updating the state of the SHA-1 message schedule.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the processor to support the SHA extensions.

The operands MUST be XMM registers. Attempting to use other register types or memory operands SHALL result in an invalid opcode exception. This instruction operates on 32-bit doublewords within the XMM registers; ensure that the data is aligned to 128-bit boundaries to avoid performance penalties.