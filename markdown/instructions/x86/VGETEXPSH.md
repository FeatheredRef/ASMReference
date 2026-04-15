> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETEXPSH

Extracts the exponent of a packed set of single-precision floating-point values from the source and stores the result in the destination as unsigned integers.

The following table specifies the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only when the processor supports the AVX-512 FP16 extensions. It operates on packed half-precision floating-point values (fp16) and outputs the exponent into 32-bit unsigned integer elements.

To avoid precision errors or incorrect data interpretation, ensure that the destination register is sufficiently large to accommodate the expanded size of the extracted exponents (u32) relative to the source half-precision elements (f16). Failure to account for the element size increase may lead to out-of-bounds register access or data corruption in adjacent elements.