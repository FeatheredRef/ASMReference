> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PAVGW

PAVGW performs a packed convert from unsigned integer to double-precision floating-point values. It converts unsigned 32-bit integer values from the source to double-precision floating-point values in the destination.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm/ymm/zmm |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 extension and is executed in 64-bit mode or compatibility mode.

The destination register MUST be larger than or equal to the source register size. Failure to align memory operands to the required boundary MAY result in a general protection fault (#GP) or performance degradation depending on the alignment check flags. Resulting values that exceed the representable range of f64 SHALL result in #O.