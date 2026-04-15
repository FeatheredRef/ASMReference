> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EAUG]

Closes an enclave. If the enclave is the currently executing enclave, it terminates the enclave and resumes execution outside the enclave. If the enclave is not the currently executing enclave, the instruction is ignored.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| reg | #I |
| imm | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It SHALL be executed within an enclave to terminate that enclave; executing it outside of an enclave context results in no operation.

The destination operand SHALL be a memory operand containing the enclave's address. If the address provided in the operand does not match the address of the currently executing enclave, the instruction SHALL NOT terminate the enclave. Failure to provide the correct enclave address will result in the instruction being ignored without triggering an exception.