> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SERIALIZE

The SERIALIZE instruction serializes the instruction stream. It ensures that all previous instructions have completed execution and that all subsequent instructions wait until the SERIALIZE instruction has retired before they begin execution.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The SERIALIZE instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

To avoid pipeline stalls or incorrect execution order in sensitive synchronization primitives, the developer SHALL ensure that the processor supports the SERIALIZE instruction before execution, as it is a recent addition to the ISA. Failure to do so will result in an invalid opcode exception (#UD).