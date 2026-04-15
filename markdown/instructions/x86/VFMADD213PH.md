> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD213PH

Computes the fused multiply-add operation of half-precision floating-point values. It calculates the product of two operands and adds a third operand to the result: $dest = (src1 \times src2) + src3$. The operation is performed with a single rounding step at the end.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg, m16 | reg |
| reg, reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 Fused Multiply-Add (FMA) and AVX-512 FP16 extensions to be supported by the processor.

To avoid precision loss and unexpected behavior, the user SHALL ensure that the floating-point control word is correctly configured. The operation may trigger the following exceptions based on the result: #I, #O, #U, and #P. Because it is a fused operation, intermediate overflow or underflow during the multiplication does not trigger an exception; only the final result is checked against the floating-point status flags.