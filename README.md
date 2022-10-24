# Project Goal

The goal of this project was to see how Rust compares to Javascript when it comes to executing a O(n) algorithm.

## Problem

Given the head of a singly linked list, reverse the list.

## Overview

In order to attempt to keep a fair comparison a linked list data structure was created for both langauges as opposed to relying on a pre-existing structure (array for JS) or built in structure (Linked List for Rust).

### Javascript

This assumes you have Node installed. If not, please install Node.

1. Run `npm install`
1. Run `cd src`
1. Run `node cli.js`
1. Follow the prompts to generate a linked list and the script will reverse the list and log execution time.

### Rust

Most likely you don't have Rust installed. Follow the instructions to install it [here](https://www.rust-lang.org/tools/install)

1. Run `cd src` (if you are not already in the src folder)
1. Run `rustc cli.js` (this builds an executable)
1. Run `./cli`
1. Follow the prompts to generate a linked list and the script will reverse the list and log execution time.

### Comparison

The previous two sections are best observed if you open two terminals so you have a side by side comparison and use the same value for the linked list size.

