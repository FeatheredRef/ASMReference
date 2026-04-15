> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHA1NEXTE

SHA1NEXTE performs a SHA-1 message schedule update. It computes the next four words of the SHA-1 message schedule using the specified source and destination registers, implementing the expansion function defined in FIPS PUB 180-1.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the processor to support the SHA extension.

The instruction operates on 128-bit XMM registers. Failure to align memory operands (if applicable to related SHA instructions in the set) may result in performance penalties, although SHA1NEXTE specifically utilizes register-to-register operands. Users MUST ensure that the SHA extension feature flag is enabled in the CPUID to avoid an `#UD` exception.