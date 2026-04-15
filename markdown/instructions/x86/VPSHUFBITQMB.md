> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSHUFBITQMB

This instruction shuffles bits from a source quadword using a control mask. It operates on 64-bit elements, where for each element, the bits are permuted according to the indices specified in the immediate operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m64/xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 VBMI (Vector Bit Manipulation Instructions) extension to be supported by the processor.

To avoid undefined behavior or illegal instruction exceptions, the user MUST ensure the processor supports the AVX512VBMI feature flag. Using this instruction on a processor that does not implement VBMI will result in an invalid opcode exception.