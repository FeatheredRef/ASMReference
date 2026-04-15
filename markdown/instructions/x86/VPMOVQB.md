> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVQB

Moves bytes from a source to a destination, zero-extending each byte to a qword.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction requires AVX support. It is not available in compatibility mode.

The destination register is overwritten. Ensure the source is aligned to the required boundary to avoid performance penalties or general protection faults depending on the memory addressing mode used.