# Conduit Fish terminal integration.

if set -q CONDUIT_FISH_INTEGRATION
    exit
end

set -gx CONDUIT_FISH_INTEGRATION 1

function __conduit_fish_command_start --on-event fish_preexec
    printf '\033]133;A\007'
end

function __conduit_fish_command_end --on-event fish_postexec
    printf '\033]133;D;%s\007' $status
end

function __conduit_fish_prompt_start --on-event fish_prompt
    printf '\033]7;file://%s%s\007' (hostname) "$PWD"
    printf '\033]133;B\007'
end

printf '\033]7;file://%s%s\007' (hostname) "$PWD"
