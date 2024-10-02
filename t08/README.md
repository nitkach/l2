### Task

It is necessary to implement your own UNIX shell utility with support for a number of simple commands:

    cd <args> - change directory (this and that can be used as an argument)

    pwd - show the path to the current directory

    echo <args> - output the argument to STDOUT

    kill <args> - "kill" the process passed as an argument

    ps - outputs general information about running processes in the format of process id, name, running time in ms.

It is also necessary to support the functionality of fork/exec commands

Additionally, it is necessary to support the pipeline on pipes (linux pipes, example cmd1 | cmd2 | .... | cmdN).
