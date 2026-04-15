> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EMODPR]

ENCLS (Enter Conditional Cloaking State) is used to enter a conditional cloaking state by specifying a target state via the operand. The instruction transitions the processor into a state where memory accesses are restricted based on the provided parameters, typically used in conjunction with Intel® SGX.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction SHALL only be executed in CPL3. It is NOT available in compatibility mode. The instruction requires the processor to be in a state that supports the specific cloaking extension; otherwise, it SHALL trigger a general protection exception (#GP).

To avoid unexpected behavior or processor exceptions, the user MUST ensure that the operand provided matches a valid state transition value defined by the hardware implementation. Providing an invalid immediate value SHALL result in an undefined state or a #GP.