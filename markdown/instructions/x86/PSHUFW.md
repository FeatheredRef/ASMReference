> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSHUFW

This instruction shuffles 16-bit words from a source XMM register based on an immediate byte. For each word in the destination, the instruction selects a word from the source XMM register as specified by the corresponding 2-bit field in the immediate.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the SSE2 instruction set extension.

The immediate value is used as an index; if the 2-bit field in the immediate specifies an index that exceeds the number of words available in the source register (index $\ge$ 8), the resulting word in the destination is set to zero. This behavior is critical to avoid unexpected data leakage or garbage values when using immediate indices.