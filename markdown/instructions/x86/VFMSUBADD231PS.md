> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD231PS

Computes the product of the first two source operands, subtracts the result from the third source operand, and adds the result to the destination operand. Specifically, it performs the operation: $dest = dest + (src3 - (src1 \times src2))$ for each packed single-precision floating-point value.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m32 |
| m32 | reg |
| m32 | m32 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX-512 foundation extension to be supported by the hardware.

The instruction utilizes the VEX/EVEX encoding; if used in a context where the destination register is also a source register, the original value of the destination is preserved for the addition. Improper alignment of memory operands may result in performance penalties or general protection faults depending on the alignment check (AC) flag in the EFLAGS register.

The operation may generate floating-point exceptions including #D, #O, #U, and #P based on the MXCSR register settings. If the result is an infinity or NaN, it follows the IEEE 754 standard for floating-point arithmetic.