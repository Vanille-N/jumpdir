#!/bin/bash

_jumpdir()
{
    local cur prev words cword
    _init_completion || return


    local IFS=$'\n' i j k

    compopt -o filenames

    COMPREPLY=($( jumpdir "$cur" ))
    return
}
if shopt -q cdable_vars; then
    complete -v -F _jumpdir -o nospace cd pushd
else
    complete -F _jumpdir -o nospace cd pushd
fi
