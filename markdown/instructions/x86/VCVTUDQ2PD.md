> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUDQ2PD

Converts an unsigned 64-bit integer (quadword) to a double-precision floating-point number.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m64 | f64 |
| x64 | f64 |

DO NOT support LOCK

This instruction is available in 64-bit mode. It is NOT supported in compatibility mode.

To avoid precision loss, be aware that since a double-precision floating-point format (f64) has a 53-bit significand, integers with a magnitude greater than $2^{53}$ MAY be rounded. The rounding is performed according to the rounding control bits in the MXCSR register.

The instruction MAY trigger the following exceptions based on the result of the conversion:
- #P: If the result requires rounding.