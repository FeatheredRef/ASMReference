> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INVD

Invalidates internal caches.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is available in all modes of operation. It is a privileged instruction and SHALL only be executed in Ring 0; otherwise, it triggers a General Protection Fault (#GP).

The `INVD` instruction does not maintain cache coherency. It invalidates the cache without writing modified data back to main memory. Therefore, any modified data in the cache that has not been written back to memory will be lost. This instruction SHALL NOT be used in SMP (Symmetric Multiprocessing) environments as it does not broadcast the invalidation to other processors. For SMP environments, `WBINVD` SHALL be used instead.