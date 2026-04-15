> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVNTDQA

Moves aligned data from memory to a register, bypassing the cache (non-temporal load).

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or 32-bit mode. The memory operand MUST be aligned on a 16-byte boundary; failure to do so SHALL result in a general-protection exception (#GP).

To avoid performance degradation, this instruction SHOULD be used when the data being loaded is not expected to be reused shortly, as it minimizes cache pollution. Because it is a non-temporal load, the processor DOES NOT maintain the typical cache hierarchy for this operation. If the data is required immediately after the load in a way that requires strong memory ordering, explicit memory barriers MAY be necessary to ensure visibility.