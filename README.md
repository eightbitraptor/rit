# rit

A Git implementation in Rust

## Why

A few of us at work are participating in a book club of [James Coglan's Building Git book](https://shop.jcoglan.com/building-git/) and I decided that I would like to attempt to follow along with the code in Rust.

## Current state

This code will init a blank repo and create a commit.

It's not possible to push to a remote with this code, it doesn't even understand branches. So whilst I have obviously used git to push this code to github, I am trying to keep to the spirit of the book, by using this repo for all the git operations that I can.

## Getting it working

You need Rust stable installed. I used this:

```
16:52:07 (mattvh@tomoe) ~ % rustc --version
rustc 1.37.0 (eae3437df 2019-08-13)
```
Head to https://rustup.rs/ if you don't have Rust yet

Then clone this repository somewhere and `cd` into it.

```
mv .git .git.real
cargo run -- init .
echo "Initial Commit" | cargo run -- commit
```
