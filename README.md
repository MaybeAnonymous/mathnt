# mathn't
computer program made to be stupid!

---

this program has only been tested on linux, you can use alternate compilers(c++ edition) if you use windows, macos, or freebsd.

# python version:

the python version does not need to be compiled, just run it using `./mathntpy.py`

if it does not execute, use `chmod +x mathntpy.py`

you can remove the `.py` of the python file if you want to(unix systems)

theoretically you should be able to use the python version with windows(or macos).


# how to compile it (c++ version):

**dependencies:**

git, g++, sudo perms(for installing it, this is optional.)

global install:
```sh
git clone https://github.com/MaybeAnonymous/mathnt.git && cd mathnt && g++ -o mathnt mathnt.cpp && sudo cp mathnt /usr/bin
```
create the binary without installing:
```sh
git clone https://github.com/MaybeAnonymous/mathnt.git && cd mathnt && g++ -o mathnt mathnt.cpp
```
