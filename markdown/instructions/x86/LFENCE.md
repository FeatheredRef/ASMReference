> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LFENCE

Serializes the instruction stream so that all previous instructions preceding the LFENCE instruction are locally completed before any instruction following the LFENCE instruction is retired. It acts as a barrier to prevent speculative execution of subsequent instructions.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

LFENCE is available in 32-bit mode, 64-bit mode, and compatibility mode. It does not require any specific operand and behaves as a no-operation (NOP) on processors that do not implement the serialization behavior.

To avoid speculative execution vulnerabilities (such as Spectre), LFENCE SHALL be used to ensure that conditional branches are resolved before subsequent memory accesses are executed. Failure to place LFENCE correctly in a sequence involving bounds checking MAY allow the processor to speculatively execute instructions with out-of-bounds indices.