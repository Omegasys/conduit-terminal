# Conduit Zsh terminal integration.
#
# Shell history remains owned by Zsh. Conduit observes command lifecycle
# and terminal state through integration sequences.

if [[ -n "${CONDUIT_ZSH_INTEGRATION:-}" ]]; then
    return 0
fi

export CONDUIT_ZSH_INTEGRATION=1

__conduit_zsh_preexec() {
    print -Pn "\e]133;A\a"
}

__conduit_zsh_precmd() {
    local status="$?"

    print -Pn "\e]133;D;$status\a"
    print -Pn "\e]7;file://${HOST:-localhost}$PWD\a"
    print -Pn "\e]133;B\a"
}

autoload -Uz add-zsh-hook

add-zsh-hook preexec __conduit_zsh_preexec
add-zsh-hook precmd __conduit_zsh_precmd

print -Pn "\e]7;file://${HOST:-localhost}$PWD\a"
