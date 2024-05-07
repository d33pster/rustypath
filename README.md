# rustypath

Path crate for rust. Managing paths made easy.

## Overview

Coverting and Managing paths in rust is now made easy.

## Table of contents

- [Add to your project](#add-to-your-project)
- [Usage](#usage)

## Add to your project

```bash
cargo add rustypath
```

## Usage

```rust
use rustypath::RPath;

// to create a new RPath
let rpath = RPath::new_from("/abc");

// to create from Path (std::path::Path)
let demopath = Path::from("/abc");
let rpath = RPath::new_from_path(demopath);

// to create from PathBuf
let demopbuf = PathBuf::from("/abc");
let rpath = RPath::new_from_pbuf(demopbuf);

// join more parts to it
let new_r_path = rpath.join("bca"); //will return /abc/bca

// convert to pathbuf
let pbuf = rpath.convert_to_pathbuf();

// convert to string
let string = rpath.convert_to_string();

// convert from relpath to abspath
let abspath = rpath.expand()

// convert from Option<PathBuf> to PathBuf
// suppose u get some output in Option<Pathbuf> 
let pbuf = RPath::convert_optionpathbuf_to_pbuf(p:Option<PathBuf>);

// get current dir in RPath
let current_working_dir = RPath::pwd();

// get home dir in RPath
let homedir = RPath::gethomedir();

// get clone
let rpath2 = rpath.clone();
```