# compiler-from-scratch
Trying to learn how compilers work!

# How to use
Trying this out, either get the binary from the releases tab or   
clone the repo and run cargo build --release ( you should have cargo installed) you'll find the binary in target/release/.

You can try checking out the write_a_c_compiler/stage_3/valid folder it has some example c programs that my compiler can compile!
I have not yet implemented a lot of c functionality tho!
run ./binary-name c-file-name.c
take an example file from write_a_compiler/stage_3/valid folder and run it!
it will compile the biniary in the directory where the source file is go there and run ./c-file-name
then look at the return code echo $? that should give you the correct return code!
For windows, if people could try it on wsl it you would be nice, cause I havent tested on windows :(  
But if you are trying on windows to see result you have to run echo %ERRORLEVEL% instead of echo $?, please run the binary in the command line also, thx.
You can also read ./c-file-name.s to see the assembly produced, also you need to have gcc installed since I dont have my own assembler.
