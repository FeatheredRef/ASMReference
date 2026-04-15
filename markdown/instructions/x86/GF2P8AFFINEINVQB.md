> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GF2P8AFFINEINVQB

This instruction computes the inverse affine transformation for each byte in a 128-bit quadword over the finite field $\text{GF}(2^8)$. It transforms the input bytes using a specified affine transformation inverse based on the constants defined by the AES standard.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor supports the GALILEO feature set. It operates exclusively on XMM registers; memory operands are not supported.

To avoid undefined behavior, ensure that the destination register is not used as a source for other dependent operations within the same pipeline stage to prevent hazards, although the hardware handles data dependencies. The operation is performed on a per-byte basis across the 128-bit register, meaning the transformation of one byte does not affect others.