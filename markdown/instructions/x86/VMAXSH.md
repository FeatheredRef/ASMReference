> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMAXSH

Computes the maximum of two signed integers and stores the result in the destination. The instruction operates on packed signed integers of the specified size within YMM or XMM registers.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm | xmm/ymm |
| imm | xmm/ymm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX extension to be enabled in the processor.

The instruction operates on signed integers; using it on unsigned data will produce mathematically incorrect results due to the sign-bit interpretation of the most significant bit. Ensure that the destination register is of the same size as the source to avoid undefined behavior or data corruption in the upper bits of the register.