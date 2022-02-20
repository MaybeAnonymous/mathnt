# mathn't
A computer program made to be stupid!

---

This program has only been tested on Linux, you can use alternate compilers(c++) if you use Windows, MacOS, or BSD.

## .py:

The python version does not need to be compiled! Just run it using `./mathntpy.py`

If it does not execute, use `chmod +x mathntpy.py`

*Theoretically* you should be able to use the python version with Windows, MacOS, or BSD.


## .cpp:

**dependencies:**

git, g++, sudo perms(optional)

Global installation:
```sh
git clone https://github.com/MaybeAnonymous/mathnt.git && cd mathnt && g++ -o mathnt mathnt.cpp && sudo cp mathnt /usr/local/bin
```
Create the binary without installing:
```sh
git clone https://github.com/MaybeAnonymous/mathnt.git && cd mathnt && g++ -o mathnt mathnt.cpp
```

# .rs

**dependencies:**

git, rustc, sudo perms(optional)

Global installation:
```sh
git clone https://github.com/MaybeAnonymous/mathnt.git && cd mathnt && rustc mathntrs.rs && sudo cp mathntrs /usr/local/bin
```
Create the binary without installing:
```sh
git clone https://github.com/MaybeAnonymous/mathnt.git && cd mathnt && rustc mathntrs.rs
```
