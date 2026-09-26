# Conduit Zsh integration.

if [[ -n "${CONDUIT_ZSH_LOADED:-}" ]]; then
    return 0
fi

export CONDUIT_ZSH_LOADED=1

conduit() {
    if (( $# == 0 )); then
        print "Conduit shell integration active."
        return 0
    fi

    case "$1" in
        reload)
            if [[ -f "${HOME}/.config/conduit/config.toml" ]]; then
                print "Conduit configuration: ${HOME}/.config/conduit/config.toml"
            fi
            ;;

        pwd)
            print -r -- "$PWD"
            ;;

        workspace)
            print -r -- "${CONDUIT_WORKSPACE:-default}"
            ;;

        *)
            print -u2 -- "Unknown Conduit shell command: $1"
            return 1
            ;;
    esac
}

conduit_workspace() {
    export CONDUIT_WORKSPACE="$1"
}

conduit_title() {
    print -Pn "\e]0;$1\a"
}

conduit_notify() {
    print -Pn "\e]777;notify;$1\a"
}

conduit_clipboard() {
    local encoded
    encoded="$(print -rn -- "$1" | base64 | tr -d '\n')"

    print -Pn "\e]52;c;$encoded\a"
}
