> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPDPBUSDS

Computes the dot product of unsigned 8-bit integers from two source operands, accumulates the result into a destination register. Specifically, it multiplies corresponding u8 elements from the first source and second source, then adds the products to the corresponding i32 elements in the destination register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm | zmm/ymm/xmm |
| m16/m32/m64 | zmm/ymm/xmm |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX-512 VNNI (Vector Neural Network Instructions) CPUID leaf.

To avoid incorrect results, the programmer MUST ensure that the destination register is initialized to the desired starting accumulation value, as the instruction performs an additive update to the existing values in the destination register. Alignment of memory operands SHALL follow the standard AVX-512 alignment requirements to avoid performance degradation or general protection faults.