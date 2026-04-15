> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD132SH

Performs a fused multiply-subtract operation on a set of floating-point values. It computes the result as $dest = (src1 \times src2) - src3$ and stores the result in the destination. The "132" notation indicates that the first operand is the destination, and the second and third operands are the multipliers, while the third operand is the subtrahend.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| fN (reg) | fN (reg) |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 foundation instructions. It SHALL be executed in 64-bit mode or compatibility mode.

To avoid precision loss or unexpected exceptions, be aware that the operation is performed with infinite precision before rounding to the destination format. The instruction may trigger #D, #O, #U, or #P depending on the floating-point control word and the magnitude of the operands. Ensure that the appropriate AVX-512 registers are initialized to prevent the use of undefined values.