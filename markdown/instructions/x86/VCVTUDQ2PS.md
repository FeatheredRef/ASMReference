> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUDQ2PS

Converts four unsigned 64-bit integers from a qword-aligned memory location or an XMM register to four single-precision floating-point numbers.

The following table describes the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| m256 (qword-aligned) | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

To avoid `#O` exceptions or precision loss, ensure the input u64 values are within the representable range of a single-precision float. Since the conversion is from unsigned 64-bit integers, values exceeding the maximum precision of the f32 format will be rounded according to the rounding control bits in the MXCSR register, potentially resulting in `#P`. Memory operands MUST be 256-bit aligned to avoid alignment check exceptions or performance penalties.