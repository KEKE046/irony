# Cmt-interpret

## Methodology

### Startup

* Module: port; instantiation of submodule
  * e.g. module A { instance @B; instance @B} instantiates module B twice as a submodule in module A
* Sequential: extract compreg registers in IR to link to the clock
* Combination: identify "dependencies"
  * Data dependency: def-use relation, DAG
  * Conditional dependency: determine the conditions under which a signal is used
    * rst = (a==0) ? (b) : (c).  
    * b is used under the condition that a==0.
* Topological ordering according to "data dependencies".

### Interpret

* Cycle-by-cycle execution
* Execution in topologically sorted order
* Submodules are viewed as instructions to be executed
* Determine whether to skip based on conditional dependencies
* Port into channel to check if current cycle data is provided
* Output results
  * Port value
  * Observe the signal

## Data structure



