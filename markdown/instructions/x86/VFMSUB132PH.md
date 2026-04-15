> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB132PH

Performs a fused multiply-subtract operation on packed half-precision floating-point values. The instruction computes the result of $(a \times b) - c$ for each corresponding element in the source operands. The operand $a$ is the first source, $b$ is the second source, and $c$ is the third source (the destination operand).

The following table specifies the supported source and destination types.

| source | destination |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available if the processor supports the AVX-512 FP16 extensions. It requires a processor that implements the `AVX512_FP16` CPUID leaf. It operates only in 64-bit mode or 32-bit mode when the appropriate extension state is enabled.

The result of the intermediate product $(a \times b)$ is calculated with infinite precision before the subtraction of $c$ is performed; this prevents rounding errors that would occur if the multiplication and subtraction were executed as separate instructions.

If the instruction is executed with a masking register (k-register), the elements that are not masked will retain their original values in the destination register. 

Floating-point exceptions may be triggered based on the MXCSR register settings:
- #O: Result exceeds the maximum representable value of a half-precision float.
- #U: Result is smaller than the minimum representable non-zero value.
- #P: The result requires rounding to fit the half-precision format.
- #D: An operand is a denormalized value.