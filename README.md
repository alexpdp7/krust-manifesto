# Introduction

This is a Rust library to simplify the generation of Kubernetes manifests.
This is an internal project, the only documentation is the code itself and [the projects I have that use it](https://github.com/alexpdp7/talos-check/blob/main/manifest-builder/src/main.rs).

# Rationale

I find writing Kubernetes manifests a tedious task.
Kubernetes manifests are flexible, but there are no shortcuts for common patterns.

I researched [Jsonnet](https://jsonnet.org/), [Dhall](https://dhall-lang.org/), and [Starlark](https://github.com/bazelbuild/starlark).
They are configuration languages that can build nested structures like Kubernetes manifests.
In fact, these languages often include abstractions for common Kubernetes patterns, such as [k8s-libsonnet](https://tanka.dev/tutorial/k-lib#k8s-libsonnet).
However, I did not want to learn a new language for this purpose, although these languages have unique advantages.

I did a prototype using Python.
However, it was not enjoyable.

I decided to try using Rust.
I discovered the [k8s-openapi](https://github.com/Arnavion/k8s-openapi) Rust model implementation.
This implementation uses a custom code generator that results in much more ergonomic code.

Writing code that uses k8s-openapi is mostly enjoyable, but Kubernetes resources are still deeply-nested and require instantiating many Rust structs.
I wrote this library to create Kubernetes resources in few function calls, implementing common abstractions.

Rust is not perfectly suited for this purpose, so I do not recommend using this library in general, but if you like Rust, the benefits might outweigh the drawbacks.
