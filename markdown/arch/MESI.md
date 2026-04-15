# MESI

> Ensures [L-Cache](</arch/l-cache>) stays coherent with [memory](</arch/memory>).

With the introduction of [multi-threading](</arch/multi-threading>), multiple processes could run in parallel and interact with the same [memory](</arch/memory>) locations (obviously). As [L-Cache](</arch/l-cache>) is not the memory per se, but a replication, it can be set into an incoherent state.

MESI being the protocol that ensures the global correctness. There are four states that a cache line can have, they being Modified, Exclusive, Shared, and Invalid. The state "modified" is when the cache has been mutated, but changes not flushed towards [memory](</arch/memory>), "exclusive" when only a core has the memory region in the cache, "shared" when multiple cores have a [memory](</arch/memory>) region in cache, and invalid when the state in the cache is not coherent in comparison with [memory](</arch/memory>).

When an operation uses [memory](</arch/memory>) in the state "invalid", a cache miss is issued and the cache updated from the [memory](</arch/memory>) state. As for the other states, I did find reliable data in regards to the behavior. Thus I assume it's undocumented behavior.

This state mechanism is ensured by a core analysing other core's memory bus, therefore if a mutation is flushed into [memory](</arch/memory>), the [L-Cache](</arch/l-cache>) of the other cores will be set invalid.

In conclusion, MESI ensures [L-Cache](</arch/l-cache>) is coherent to the [memory](</arch/memory>) state.
