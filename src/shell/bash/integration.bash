# Conduit Bash terminal integration.
#
# Emits terminal control sequences that allow Conduit to observe
# shell lifecycle information without taking ownership of shell history.

if [[ -n "${CONDUIT_BASH_INTEGRATION:-}" ]]; then
    return 0
fi

export CONDUIT_BASH_INTEGRATION=1

__conduit_bash_command_start() {
    printf '\033]133;A\007'
}

__conduit_bash_prompt_start() {
    printf '\033]133;B\007'
}

__conduit_bash_command_end() {
    local status="$1"

    printf '\033]133;D;%s\007' "$status"
}

__conduit_bash_directory() {
    printf '\033]7;file://%s%s\007' \
        "${HOSTNAME:-localhost}" \
        "$PWD"
}

__conduit_bash_preexec() {
    __conduit_bash_command_start
}

__conduit_bash_precmd() {
    local status="$?"

    __conduit_bash_command_end "$status"
    __conduit_bash_directory
    __conduit_bash_prompt_start
}

if [[ -n "${PROMPT_COMMAND:-}" ]]; then
    PROMPT_COMMAND="__conduit_bash_precmd;${PROMPT_COMMAND}"
else
    PROMPT_COMMAND="__conduit_bash_precmd"
fi

if [[ -z "${BASH_PREEXEC_LOADED:-}" ]]; then
    __conduit_bash_original_debug_trap="$(trap -p DEBUG)"

    trap '__conduit_bash_preexec' DEBUG
fi

__conduit_bash_directory
