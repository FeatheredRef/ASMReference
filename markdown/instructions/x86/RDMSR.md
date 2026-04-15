> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDMSR

Reads a model-specific register (MSR) specified by the value in the ECX register into EDX:EAX.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (ECX) | reg (EDX:EAX) |
| imm | #I |
| mN | #I |

DO NOT support LOCK

This instruction SHALL only be executed at privilege level 0 (CPL=0). Execution at any other privilege level SHALL result in a general-protection exception (#GP(0)).

The value of the MSR is loaded into EDX:EAX; if the MSR is smaller than 64 bits, the upper bits of EDX SHALL be cleared. Attempting to read an undefined MSR SHALL result in a general-protection exception (#GP(0)).

To avoid #GP(0) exceptions, the software MUST verify the processor's supported MSRs via CPUID or by referring to the specific processor's architectural documentation before execution.