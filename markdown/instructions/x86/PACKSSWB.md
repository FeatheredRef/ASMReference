> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PACKSSWB

Packs signed 16-bit integers from the source into signed 8-bit integers in the destination. The operation saturates values that are outside the range of a signed 8-bit integer (between -128 and 127) to the respective minimum or maximum value.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE2 instruction set to be supported by the processor.

The instruction operates on XMM registers; using general-purpose registers or memory addresses as direct operands for the source or destination is invalid. To process data from memory, a separate `MOV` or `MOVAPS`/`MOVUPS` instruction MUST be used to load data into an xmm register first. Failure to ensure the data is correctly aligned when using aligned load instructions MAY result in a general-protection exception.