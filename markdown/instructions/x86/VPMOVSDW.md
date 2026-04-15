> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVSDW

Converts signed dword integers from a source to signed word integers in a destination. The instruction extracts the lower 16 bits of each 32-bit element from the source and stores them in the destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |
| #I | imm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX CPU feature flag to be supported by the processor.

The destination register is partially overwritten based on the number of elements converted; however, in the VPMOVSDW case, the destination xmm register is fully updated. Failure to ensure the source memory alignment may result in performance degradation.