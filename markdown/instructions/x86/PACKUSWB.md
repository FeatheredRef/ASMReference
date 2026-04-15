> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PACKUSWB

Packs unsigned bytes from a source into unsigned words in a destination. The instruction converts unsigned 8-bit integers to unsigned 16-bit integers by zero-extending each byte.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The destination register is overwritten by the packed values; therefore, any existing data in the destination xmm register that is not covered by the packed result is undefined. Users MUST ensure that the source xmm register contains valid u8 values to avoid logical errors, although the instruction will technically zero-extend any bit pattern present in the byte lanes.