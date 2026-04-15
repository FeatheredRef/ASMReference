> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVUSQD

Moves unsigned quadword integers from a source to a destination, converting them to signed quadword integers.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m64 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512F instruction set extension to be supported by the processor.

Ensure that the destination register size matches the intended operation to avoid unintended data truncation or overlap when using masked versions of this instruction. When accessing memory, the source address MUST be aligned to the size of the element to avoid performance penalties or faults depending on the alignment check flags.