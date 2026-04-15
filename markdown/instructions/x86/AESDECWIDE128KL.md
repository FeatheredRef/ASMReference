> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESDECWIDE128KL

Performs a single round of the AES-128 decryption process on 128 bits of data using a specified round key, operating on the lower 128 bits of a 256-bit ZMM register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m128 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction requires the AVX-512 side-channel-resistant or AES-NI extension support. It is available in 64-bit mode.

To avoid undefined behavior or faults, the destination register SHALL be a ZMM register if the instruction is used within an AVX-512 context to prevent potential alignment issues or state corruption associated with mixed SIMD widths. Ensure that memory operands are 16-byte aligned to avoid performance penalties or general protection faults.