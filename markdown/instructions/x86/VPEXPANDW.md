> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPEXPANDW

Expands 16-bit integer values from a source to a destination by expanding each word element into a double word element. The expansion is performed by zero-extending the source word into the destination double word.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m16 | reg |

DO NOT support LOCK

This instruction requires AVX-512 support. It is NOT available in compatibility mode. It operates on zmm or ymm registers; usage of ymm registers triggers the VEX-encoded instruction behavior, while zmm registers use EVEX-encoding.

The destination register MUST NOT overlap with the source register unless the specific masking implementation allows for it. When using write masking (k-mask), elements that are masked are not updated in the destination, preserving their original values. Ensure the destination register is sufficiently large to accommodate the expanded double word elements to avoid out-of-bounds memory access if using memory destinations (though this instruction primarily targets registers for destination).