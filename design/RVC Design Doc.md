# Rust Version Control (RVC)

## Foreword

This project is strictly for my own learning purposes. I have never made a version control system before and my experience with them begins with Subversion and ends with Git. I have absolutely no illusions that what I produce here will be superior or even coming close to replicating what the two aforementioned softwares are capable of doing. My interest only lies in this being an interesting learning experience!

## Phase 1

### Plan

I believe it makes the most sense to approach this project from an iterative design approach. The first version of this I'd like to make is quite simple: it's a CLI program which is capable of the following:

* maintaining repository information in a `/.rvc` directory
* keeping version history as a series of full repository snapshots
* restoring to a designated snapshot upon request

### Implementation

Like other Rust CLI programs, there will be a lightweight `main.rs` from which the program will be access, with the majority of the runtime living in` lib.rs`. It will be prudent to separate out into other files as necessary (such as `error.rs` for error handling), but the limit of usefulness OOP can provide for this project is dubious.

Let's break down the problem:
 
> maintaining repository information in a `/.rvc` directory

In this case, we probably want `Repo` object to act as the parser of information in `/.rvc`. On the other hand, not a lot needs to be understood by the program at runtime. Let's suppose some simpler scenarios:

* We want to make the initial commit

The program should intuitively understand that if it is within any part of the repo, that it must look for `/.rvc` at the root.

* We want to make any commit

The program must now additionally know what the latest previous version was and how to uptick that for this new version