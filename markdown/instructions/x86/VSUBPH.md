> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSUBPH

Subtracts two packed half-precision floating-point values.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m16/m32/m64 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 FP16 extension. It operates on registers in 64-bit mode or compatibility mode.

The destination register is overwritten by the result; therefore, if the destination is also a source, the original value is lost. When using the memory operand, the alignment of the memory region SHALL be compatible with the vector size to avoid performance penalties or faults depending on the alignment check settings.