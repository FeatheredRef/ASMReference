> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GETSEC[ENTERACCS]

Retrieves the current access rights for the Enclave Page Cache (EPC) and stores the result in the specified register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |
| #I | r64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in Long Mode and SGX is enabled. Execution outside of an enclave or in a non-SGX enabled environment SHALL result in an undefined operation (#UD).

The instruction relies on the current processor state and the SGX Enclave Page Cache Map (EPCM). If the processor is not currently executing within an enclave context, the returned value SHALL indicate that no enclave access rights are active. Ensure that the destination register is a 64-bit general-purpose register to avoid data truncation.