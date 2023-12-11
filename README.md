# context-version-exec

The overall purpose of this repo is just to experiment with some CLI ideas in Rust. The goal here is to create a command line application that will intelligently decide if the running version is capable of handling the task in the current working directory or not. If it can, the execution will continue as is, if it cannot it will [exec](https://en.wikipedia.org/wiki/Exec_(system_call) a different program. 
