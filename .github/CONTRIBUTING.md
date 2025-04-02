# Contributing

## Request for changes/ Pull Requests

You first need to create a fork of the [roblox-template](https://github.com/iceeburr/roblox-template/) repository to commit your changes to it. Methods to fork a repository can be found in the [GitHub Documentation](https://docs.github.com/en/get-started/quickstart/fork-a-repo).

Then add your fork as a local project:

```sh
git clone https://github.com/iceeburr/roblox-template
```

> [Which remote URL should be used ?](https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories)

Then, go to your local folder

```sh
cd roblox-template
```

Add git remote controls :

```sh
git remote add fork https://github.com/YOUR-USERNAME/roblox-template
git remote add upstream https://github.com/iceeburr/github-issue-template
```

You can now verify that you have your two git remotes:

```sh
git remote -v
```

## Receive remote updates

In view of staying up to date with the central repository:

```sh
git pull upstream main
```

## Choose a base branch

Before starting development, you need to know which branch to base your modifications/additions on. When in doubt, use main.
When creating a new branch, name it with useful information. For example, if your pull request fixes a bug, name it bugfix-20.
This way it is clear that this branch fixes issue #20 which is a bug.

```sh
# Switch to the desired branch
git switch master

# Pull down any upstream changes
git pull

# Create a new branch to work on
git switch --create 1234-name-issue
```

Commit your changes, then push the branch to your fork with `git push -u fork` and open a pull request on [the roblox-template repository](https://github.com/iceeburr/roblox-template/) following the template provided.

While not always enforced, it is recommended to follow the code style guide provided. As this is only a template repository, we can only suggest you what to use. The boilerplate code we have follows the official Roblox Lua style guide and the Lua Rocks style guide. We use the default and recommended 4 tab spaces for indentation. All of that is up to your own preference and has no real meaning. You can read both of them here: <br>
https://roblox.github.io/lua-style-guide/ <br>
https://github.com/luarocks/lua-style-guide
