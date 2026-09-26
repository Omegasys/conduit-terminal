# Conduit Fish integration.

if set -q CONDUIT_FISH_LOADED
    exit
end

set -gx CONDUIT_FISH_LOADED 1

function conduit
    if test (count $argv) -eq 0
        echo "Conduit shell integration active."
        return 0
    end

    switch $argv[1]
        case reload
            if test -f "$HOME/.config/conduit/config.toml"
                echo "Conduit configuration: $HOME/.config/conduit/config.toml"
            end

        case pwd
            pwd

        case workspace
            echo $CONDUIT_WORKSPACE

        case '*'
            echo "Unknown Conduit shell command: $argv[1]" >&2
            return 1
    end
end

function conduit_workspace
    set -gx CONDUIT_WORKSPACE $argv[1]
end

function conduit_title
    printf '\033]0;%s\007' "$argv[1]"
end

function conduit_notify
    printf '\033]777;notify;%s\007' "$argv[1]"
end

function conduit_clipboard
    set -l encoded (printf '%s' "$argv[1]" | base64 | string collect | string replace -a '\n' '')

    printf '\033]52;c;%s\007' "$encoded"
end
