# UCB Tool
This project is a mock-up of a compiled CLI tool.

The public repository is an example of how developers are free to access this source code and raise any issues that need fixing and/or push updates/fixes themselves.

## Envisioned workflow:
1. Developers build the first release and can test it locally (in isolation) on their machines
2. Any version of the tool can only be put into production by having it approved and merged into the `main` branch 
3. In order to be merged into the `main` branch, manual reviews/approvals must be granted by explicitly defined people
4. Once approved, the compiled binary can be placed where required (eg a directory under `$PATH`)

## Example execution:
```
>  ucb-tool
```

![Alt Text](https://res.cloudinary.com/bendray/image/upload/v1615977136/2021-03-17_11-27-42_osudz5.gif)