> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSRAD

The PSRAD instruction performs a pseudorandom sequence generation by updating a seed value and returning the generated value. It utilizes a linear feedback shift register (LFSR) mechanism to produce a sequence of pseudo-random numbers.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | reg |
| mN | #I |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

The seed register MUST be properly initialized before the first call to PSRAD to avoid producing a predictable sequence. Failure to provide a unique seed across different process instances WILL result in identical pseudo-random sequences.