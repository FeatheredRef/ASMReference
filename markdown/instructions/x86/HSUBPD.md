> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# HSUBPD

Subtracts two double-precision floating-point packed values. The instruction subtracts the second operand from the first and stores the result in the destination.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE2 instruction set extension to be supported by the processor.

Ensure that the destination register is not used as a source for other simultaneous operations if the result is needed for subsequent calculations. This instruction operates on packed data; if the input vectors are not properly aligned to 16-byte boundaries when using memory operands, a general protection exception (#GP) may occur depending on the specific processor implementation and memory alignment checks.