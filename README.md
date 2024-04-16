# locker

Asset management in Git for humans

## NOTE

Everything in here is extremely WIP as this is a learning project and also I have no immediate need to finish it.

## Installation

### Brew

todo

## Getting Started

### Admin

#### Setting your workflow strategy

Right now Locker has two available strategies for workflows:
> Workflow customization is coming soon!

##### Default

Controlled workflow mode will use the built-in strategy for managing branches, commits, and pushes.
> todo - image of strategy

Claims are tied to both the user and the workspace. A user claim may be released
only to a workspace. A workspace must be completed (or files returned with no changes)
for claims to be released back to the repo.

##### None

With `workflow` set to `none` Locker will

#### Workspace Options

Workspace options are machine local

##### Separated Workspace

1. On new workspace will create an ignored folder at root or at specified path.
2. Claimed files will have symlinks created in the folder.
3. New files will UNKNOWN

##### Monitoring

#### Recommended Repo Settings

##### Github

todo

##### Gitlab

todo

##### OTHER

todo

# Dependencies

- Git LFS
- git