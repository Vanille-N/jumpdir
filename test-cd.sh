_test-cd()
{
    local cur="$1" prev="$2" words="$3" cword="$4"
    _init_completion || return

    #echo "-<$cur>- -<$prev>- -<$words>- -<$cword>-"

    local IFS=$'\n' i j k
    COMPREPLY=()

    # Use standard dir completion if no CDPATH or parameter starts with /,
    # ./ or ../
    if [[ -z "${CDPATH:-}" || "$cur" == ?(.)?(.)/* ]]; then
        _filedir -d
        echo "${COMPREPLY[@]}"

        return
    fi

    local -r mark_dirs=$(_rl_enabled mark-directories && echo y)
    local -r mark_symdirs=$(_rl_enabled mark-symlinked-directories && echo y)

    # we have a CDPATH, so loop on its contents
    for i in ${CDPATH//:/$'\n'}; do
        # create an array of matched subdirs
        k="${#COMPREPLY[@]}"
        for j in $(compgen -d -- $i/$cur); do
            if [[ ( $mark_symdirs && -h $j || $mark_dirs && ! -h $j ) && ! -d ${j#$i/} ]]; then
                j+="/"
            fi
            COMPREPLY[k++]=${j#$i/}
        done
    done

    _filedir -d

    if [[ ${#COMPREPLY[@]} -eq 1 ]]; then
        i=${COMPREPLY[0]}
        if [[ "$i" == "$cur" && $i != "*/" ]]; then
            COMPREPLY[0]="${i}/"
        fi
    fi


    return
}
