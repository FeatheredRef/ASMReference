> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SFENCE

Serializes all store instructions that precede the SFENCE instruction. It ensures that all stores before the SFENCE are globally visible before any store following the SFENCE is executed.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

SFENCE is available in 64-bit mode, 32-bit mode, and compatibility mode. It does not require any operand; therefore, it does not affect any architectural registers or memory locations directly.

To avoid memory consistency issues in non-temporal store operations (such as those using MOVNTSS or MOVNTDQ), SFENCE SHALL be used. Without SFENCE, the processor may reorder non-temporal stores, which can lead to data corruption or incorrect synchronization in multiprocessor environments.