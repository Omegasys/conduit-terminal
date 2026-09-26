# Conduit Bash integration.
#
# This file provides the user-facing Conduit shell helpers.
# The actual prompt/command lifecycle integration lives in integration.bash.

if [[ -n "${CONDUIT_BASH_LOADED:-}" ]]; then
    return 0
fi

export CONDUIT_BASH_LOADED=1

conduit() {
    if [[ $# -eq 0 ]]; then
        printf '%s\n' "Conduit shell integration active."
        return 0
    fi

    case "$1" in
        reload)
            if [[ -f "${HOME}/.config/conduit/config.toml" ]]; then
                printf '%s\n' "Conduit configuration: ${HOME}/.config/conduit/config.toml"
            fi
            ;;

        pwd)
            printf '%s\n' "$PWD"
            ;;

        workspace)
            printf '%s\n' "${CONDUIT_WORKSPACE:-default}"
            ;;

        *)
            printf 'Unknown Conduit shell command: %s\n' "$1" >&2
            return 1
            ;;
    esac
}

conduit_workspace() {
    export CONDUIT_WORKSPACE="$1"
}

conduit_title() {
    local title="$1"

    printf '\033]0;%s\007' "$title"
}

conduit_notify() {
    local message="$1"

    printf '\033]777;notify;%s\007' "$message"
}

conduit_clipboard() {
    local data="$1"

    printf '\033]52;c;%s\007' \
        "$(printf '%s' "$data" | base64 | tr -d '\n')"
}
