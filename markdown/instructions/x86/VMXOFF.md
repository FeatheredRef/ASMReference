> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMXOFF

VMXOFF disables VMX operation. If VMX is currently enabled and the processor is operating in VMX root operation, the instruction transitions the processor to non-root operation and clears the VMX enabled state. If VMX is already disabled, the instruction is ignored.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

The instruction SHALL only be executed in VMX root operation. If executed in VMX non-root operation, a VM exit occurs. The instruction is not supported in compatibility mode.

The operand MUST be a 32-bit immediate value of 1. Any other value SHALL cause a #UD (Invalid Opcode) exception. Failure to provide the specific immediate value `1` will prevent the instruction from executing.