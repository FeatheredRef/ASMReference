> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD132PD

Computes a fused multiply-subtract product for double-precision floating-point values. It calculates $dest = (src1 \times src2) - src3$ for each 64-bit element, where the operands are ordered according to the 132 pattern (src1 and src2 are multiplied, then src3 is subtracted from the result).

The following table specifies the supported source and destination operand types.

| source | destination(s) |
| :--- | :--- |
| reg, m8 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires AVX support. If executed on a processor that does not support the AVX instruction set, an invalid opcode exception SHALL occur.

The instruction utilizes the MXCSR register to control rounding and floating-point exception handling. Precision (#P) and Overflow (#O) exceptions may be triggered depending on the result of the fused operation. Denormalized operands (#D) are handled according to the flush-to-zero (FTZ) and denormals-are-zero (DAZ) bits in the MXCSR register. Because this is a fused operation, only one rounding step is performed at the end of the calculation, which provides higher precision than performing a separate multiply and subtract.