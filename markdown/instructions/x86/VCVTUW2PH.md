> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUW2PH

Converts an unsigned word integer value to a packed half-precision floating-point value. The instruction converts the unsigned 16-bit integer source to a 16-bit floating-point destination using rounding-to-nearest-even.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m2 | reg |
| #I | imm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires support for the AVX-512 FP16 instruction set.

The conversion process utilizes the rounding mode specified in the MXCSR register. Since the source is an unsigned word (u16), the maximum value ($2^{16}-1$) fits within the representable range of a half-precision float (f16) without causing a #O exception, although precision loss may occur resulting in a #P flag.