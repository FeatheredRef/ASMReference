> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCONFIG

Configures processor-specific features by writing a value to a model-specific register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |
| imm | #I |
| mN | #I |

DO NOT support LOCK

PCONFIG is only available in 64-bit mode. Execution of PCONFIG in compatibility mode or 32-bit mode SHALL result in an invalid opcode exception.

To avoid unexpected behavior, the instruction MUST be executed with the appropriate privilege level; attempting to execute PCONFIG at CPL 3 SHALL result in a general protection fault (#GP). Additionally, users MUST ensure the processor supports the PCONFIG instruction by checking CPUID, as attempting to use it on unsupported hardware will result in an invalid opcode exception.