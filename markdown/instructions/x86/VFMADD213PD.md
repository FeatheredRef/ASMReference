> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD213PD

Computes the product of two double-precision floating-point numbers and adds the result to a third double-precision floating-point number. The operation is performed as: destination = (a * c) + b, where 'a' is the first operand, 'b' is the second operand, and 'c' is the third operand.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, m128, reg | reg |
| reg, reg, m128 | reg |
| reg, m128, m128 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX instruction set extension to be enabled in the processor.

The instruction utilizes the MXCSR register to control rounding and floating-point exception handling. If the destination register is also used as a source, the original value is overwritten. Precision exceptions (#P) may occur if the result cannot be represented exactly. Numeric overflow (#O) and underflow (#U) are handled according to the rounding mode specified in MXCSR. Denormalized operands (#D) are handled based on the Flush-to-Zero (FTZ) and Denormals-Are-Zero (DAZ) flags.