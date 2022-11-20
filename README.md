# locker
Asset management in Git for humans

## Installation
### Brew
```shell
brew update
brew install locker
```

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
- Switching workspaces sets up a cron which looks for changes

#### Recommended Repo Settings
##### Github
todo - branch protection stuff

##### Gitlab
todo

##### OTHER
todo

# TODO
- setup hooks for returning a file
  - should the lock be transferred to an admin?

# Dependencies
- Git LFS
- sed
- git