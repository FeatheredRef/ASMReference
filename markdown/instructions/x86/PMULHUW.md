> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMULHUW

Multiplies two unsigned word operands and stores the upper 16 bits of the 32-bit result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm / ymm / zmm | xmm / ymm / zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The instruction operates on packed unsigned integers; using this instruction on signed integers will yield an incorrect result as it does not perform sign extension. The destination register is overwritten by the result, so the original data in the destination register MUST be preserved in another register if needed.