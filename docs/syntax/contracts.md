## a list of the contracts and what each one does

Contracts are executable actions available to Chaos programs.

A contract receives the current value as input and performs an action. Contracts may display output, modify the environment, or produce some other observable effect.

## Current Contracts

### `print`

Prints the current input value.

Example:

```text
action ('print')
```

Input:

```text
hello
```

Output:

```text
hello
```

---

### `double`

Interprets the input as an integer and prints its value multiplied by two.

Example:

```text
action ('double')
```

Input:

```text
123
```

Output:

```text
246
```

---

### `terminate`

Terminates the current action/program flow.

Example:

```text
terminate
```

Current output:

```text
Action terminated.
```

---

### `increment`

Interprets the input as an integer and prints its value increased by one.

Example:

```text
action ('increment')
```

Input:

```text
123
```

Output:

```text
124
```

---

### `decrement`

Interprets the input as an integer and prints its value decreased by one.

Example:

```text
action ('decrement')
```

Input:

```text
123
```

Output:

```text
122
```

---

### `reset`

Ignores the current input and prints `0`.

Example:

```text
action ('reset')
```

Input:

```text
123
```

Output:

```text
0
```

---

### `clear`

Clears the terminal screen and moves the cursor to the top-left position.

Example:

```text
action ('clear')
```

---

