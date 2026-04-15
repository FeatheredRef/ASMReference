> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD213PH

Performs a fused multiply-subtract operation on half-precision floating-point values. The instruction computes the result as `(a * c) - b` and stores it in the destination. The operation is performed on packed f16 elements.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction requires the AVX-512 FP16 extension. It is not available in compatibility mode if the processor does not support the corresponding AVX-512 feature set.

The operation must be performed using the precision and rounding control specified in the MXCSR register. Incorrect configuration of the rounding mode may result in unexpected precision loss or #P. Ensure that the target processor supports the AVX-512 FP16 instruction set to avoid `#UD` (Undefined Instruction) exceptions.