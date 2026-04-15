> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CRC32

Computes the Cyclic Redundancy Check (CRC) checksum of a value using the polynomial 0x04C11DB7 and updates the accumulator.

The following table covers the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m4 | reg |
| m8 | reg |

DO NOT support LOCK

This instruction is available only if the SSE4.2 feature set is supported. It operates in 32-bit and 64-bit modes.

The destination register MUST be the same register used as the accumulator for the CRC calculation to maintain the checksum state across multiple calls. Failure to preserve the accumulator value across calls will result in an incorrect checksum.