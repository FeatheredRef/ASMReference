> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[ELDUC]

Clears the Enclave Page Cache Map (EPCM) entry for the page specified by the operand and notifies the processor that the page is no longer associated with the enclave.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| #I | reg |
| #I | m64 |

DO NOT support LOCK

This instruction is only available when the processor is executing within an enclave (Enclave Mode). It SHALL NOT be executed outside of an enclave; doing so results in a General Protection Fault (#GP).

The instruction operates on the EPCM. If the specified page is not currently owned by the calling enclave, the instruction SHALL fail and return an error code in r64. The operation is architectural and requires the processor to be in a state where SGX (Software Guard Extensions) is enabled and supported.

Ensure that the address passed to the instruction is correctly aligned to a page boundary. Passing a non-page-aligned address may lead to unexpected behavior or failure of the operation. The programmer MUST verify the return value in the destination register to confirm if the EPCM entry was successfully cleared.