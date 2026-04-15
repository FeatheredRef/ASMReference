> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPROLVD

Rotates a quadword element in a zmm register left by the number of bits specified in a second zmm register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It requires the AVX-512 foundation (AVX-512F) extension to be supported and enabled.

The rotation count is masked to 6 bits (0-63) for each 64-bit element; any bits beyond the 6th bit in the count register are ignored. Failure to mask the count register manually is not required as the hardware performs this operation. Ensure that the destination register is not used as a source for the count if the original count values must be preserved.