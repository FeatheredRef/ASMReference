> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDPKRU

RDPKRU reads the Protection Key Rights User (PKRU) register into a destination register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal PKRU Register | reg |

DO NOT support LOCK

This instruction is only available when the processor supports Protection Keys for Userspace. It is supported in 64-bit mode and compatibility mode.

The instruction is not a privileged instruction; it can be executed in CPL 3. However, if the `CR0.WP` bit is not set, certain behaviors regarding page protections may vary. 

To avoid an `#UD` (Undefined Opcode) exception, the software MUST ensure that the CPU supports the PKRU feature and that the corresponding feature flag is enabled in the processor. Use `CPUID` to verify support before execution.