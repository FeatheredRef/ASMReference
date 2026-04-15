> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XSETBV

Sets the Model Specific Register (MSR) specified by the value in the ECX register to the value contained in the EDX:EAX register pair.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

The instruction SHALL only be executed in CPL 0. Execution at any other privilege level SHALL cause a General Protection Fault (#GP). This instruction is not supported in compatibility mode.

To avoid an invalid operation or a system crash, the user MUST ensure that the value being written to the MSR is valid for that specific register. Writing reserved bits or unsupported values to an MSR MAY result in an undefined processor state or a General Protection Fault (#GP). Since XSETBV specifically handles 64-bit values via EDX:EAX, the programmer MUST ensure the register pair is correctly loaded before execution.