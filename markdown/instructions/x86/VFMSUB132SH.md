> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB132SH

Performs a fused multiply-subtract operation on signed packed 16-bit floating-point values. The instruction computes the result of $dest = (a \times b) - c$, where $a$ and $b$ are the first two source operands and $c$ is the third source operand, and stores the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction SHALL only be executed when the processor is operating in 64-bit mode or compatibility mode. It REQUIRES AVX-512 foundation and the specific AVX-512 FP16 extension support.

The destination register is SHALL be one of the first two source operands. If the destination register is not one of the sources, the operation is #I. Precision and rounding are governed by the MXCSR register; failure to set the correct rounding mode may result in #P.