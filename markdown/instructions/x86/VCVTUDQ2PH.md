> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUDQ2PH

Converts an unsigned 64-bit integer (quadword) to a 16-bit half-precision floating-point number. The conversion is performed by converting the u64 value to a floating-point representation and rounding it according to the rounding control in the MXCSR register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| imm | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX-512 FP16 extension to be supported by the hardware.

The operation may trigger #O if the unsigned integer value exceeds the maximum representable value of a half-precision float. Precision loss occurs during conversion, which SHALL set the #P floating-point exception flag in the status register if the result is not exact. Ensure the destination register is an XMM or ZMM register compatible with the FP16 data type to avoid general protection faults.