> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB213SH

Performs a fused multiply-subtract operation on signed short floating-point values. It computes the result of $(a \times b) - c$ using the provided operands, where the result is stored in the destination.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is available only when the processor supports the AVX-512 Foundation and the AVX-512 FP16 extensions. It SHALL only be executed in 64-bit mode or compatibility mode.

The operation is performed on the lower 16 bits of the elements within the zmm/ymm/xmm registers. If the destination register is also used as a source for the subtrahend ($c$), the original value SHALL be preserved until the final subtraction occurs. Precision and rounding are governed by the MXCSR register.

To avoid precision loss or unexpected exceptions, ensure that the MXCSR rounding mode is correctly configured. Operations may trigger #P, #O, #U, or #D depending on the input magnitudes and the resulting value's proximity to the representable range of f16.