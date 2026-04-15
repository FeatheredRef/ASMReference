> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TILESTORED

TILESTORED stores the contents of a specified tile register to a memory location.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | mN |

DO NOT support LOCK

The instruction SHALL only be executed when the processor is in 64-bit mode. The memory destination MUST be aligned to the requirements of the tile architecture to avoid alignment exceptions.

The instruction requires the AMX (Advanced Matrix Extensions) feature to be enabled in the CPU and the tile configuration to be initialized via TCONFIG. If the tile state is not enabled, executing TILESTORED SHALL result in an undefined operation (#I).