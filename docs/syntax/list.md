## table of chaos primitives

| Primitive        | Purpose                                                                                                                                                   |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `logic`          | Defines a process or piece of active reasoning/computation. The main unit of work in Chaos.                                                               |
| `execute`        | Runs a contract or other executable operation from within `logic`.                                                                                        |
| `contract`       | Defines a reusable operation with a known purpose and rules. Scientific libraries would largely be built from these.                                      |
| `register`       | Holds the currently active computational states available to `logic`.                                                                                     |
| `state`          | Represents a piece of information or a condition that can be stored, loaded, changed, and reused.                                                         |
| `load`           | Retrieves a state from the register/context into the current logic.                                                                                       |
| `transition`     | Changes one state into another according to defined rules. Useful for experiments, workflows, simulations, and stateful computation.                      |
| `constant`       | Stores a local value that remains available to a piece of `logic`, rather than being part of the main register.                                           |
| `context`        | Defines the environment in which a piece of logic operates, including its available information and applicable rules.                                     |
| `rule` / `rules` | Defines constraints or conditions that govern what may happen within a context.                                                                           |
| `list`           | Ordered general-purpose collection of data.                                                                                                               |
| `queue`          | Ordered collection using first-in, first-out behavior.                                                                                                    |
| `stack`          | Ordered collection using last-in, first-out behavior.                                                                                                     |
| `branch`         | Tree-like hierarchical data structure.                                                                                                                    |
| `push`           | Adds data to a storage structure such as a list, queue, stack, or branch.                                                                                 |
| `pop`            | Retrieves/removes data from a storage structure. Not used for register states.                                                                            |
| `write`          | Creates a new textual research record, note, document, etc.                                                                                               |
| `edit`           | Modifies existing textual material.                                                                                                                       |
| `change`         | Modifies structured research data at its source. Can trigger recalculation of dependent results.                                                          |
| `lookup`         | Finds/retrieves existing information from the Chaos project.                                                                                              |
| `search`         | Searches external sources, particularly the internet, for research material.                                                                              |
| `encode`         | Converts Chaos data/research results into an external representation such as a chart, table, diagram, Markdown/Mermaid, or eventually a paper/PDF format. |
| `decode`         | Extracts structured information from an encoded representation or converts one representation into another useful form.                                   |
