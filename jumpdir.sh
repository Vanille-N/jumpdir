#!/bin/bash

# Without -o nospace there is an extra unwanted space after completion
complete -C jumpdir -o nospace cd
