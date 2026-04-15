> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMWRITE

VMWRITE writes a value to a specified field of the current Virtual Machine Control Structure (VMCS).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | imm |

DO NOT support LOCK

VMWRITE SHALL only be executed in VMX root operation. Execution in VMX non-root operation SHALL result in a general-protection exception (#GP).

To avoid invalid operation errors, the immediate operand MUST specify a valid field encoding as defined by the VMCS field encoding table. Attempting to write to a read-only field or an unsupported field SHALL result in a #GP. The value loaded into the source register MUST be compatible with the size and format of the target VMCS field; otherwise, the behavior may be undefined or result in a #GP.