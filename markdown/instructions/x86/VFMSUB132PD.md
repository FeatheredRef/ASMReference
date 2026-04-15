> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB132PD

Performs a fused multiply-subtract operation on packed double-precision floating-point values. The instruction computes the result of (a * b) - c for each corresponding pair of elements in the source operands, where the operands are mapped as follows: the first source is multiplied by the second source, and the third source is subtracted from the product.

The table below covers what the source and destinations can be.

| Source | Destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| m64, reg, reg | reg |
| reg, m64, reg | reg |
| reg, reg, m64 | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or 32-bit mode with AVX enabled. It requires the VEX encoding scheme; using it in compatibility mode without AVX support will result in an invalid opcode exception.

The destination register is used as one of the operands (the third source operand `c`) and is overwritten by the result. To preserve the original value of the third operand, it MUST be copied to a temporary register before execution. This instruction performs a single rounding step at the end of the entire operation, preventing the intermediate precision loss associated with separate multiply and subtract instructions.