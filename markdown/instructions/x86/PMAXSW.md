> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMAXSW

PMAXSW compares two signed 16-bit integers and returns the maximum value. It operates on packed 16-bit signed integers.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available when the processor is in 64-bit mode or compatibility mode. It requires SSE4.1 support.

The instruction operates on packed signed 16-bit integers; using this instruction on unsigned data will result in incorrect maximum value selection due to the sign bit interpretation. Ensure that the XMM/YMM/ZMM registers are correctly aligned to the operand size to avoid performance penalties.