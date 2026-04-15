> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHA256MSG2

Performs the SHA-256 message schedule update. It computes the sum of the 16-byte (128-bit) values in the source and destination registers according to the SHA-256 message schedule algorithm, updating the destination registers with the result.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only if the SHA extension is supported by the processor. It requires the processor to be in 64-bit mode or compatibility mode.

The destination registers must be the same registers used as sources for the operation to ensure the correct state of the message schedule; using mismatched registers may result in incorrect SHA-256 hash calculations. The instruction operates on 128-bit XMM registers; attempting to use general-purpose registers will result in an invalid encoding.