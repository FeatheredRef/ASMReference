> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BLENDPD

BLENDPD blends two double-precision floating-point values from a source operand and a destination operand based on a mask specified by an immediate. If the mask bit is 1, the corresponding double-precision value from the source is selected; if the mask bit is 0, the corresponding value from the destination is retained.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

The instruction SHALL be executed in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension to be supported by the processor.

To avoid incorrect results, the user MUST ensure that the immediate mask is valid; only the least significant bit of the immediate is used for the blend operation of double-precision values in an xmm register. If the operand is a memory region, the memory alignment SHALL follow the requirements of the xmm register to avoid alignment check exceptions.