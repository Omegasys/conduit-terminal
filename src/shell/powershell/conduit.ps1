# Conduit PowerShell integration.

if ($env:CONDUIT_POWERSHELL_LOADED -eq "1") {
    return
}

$env:CONDUIT_POWERSHELL_LOADED = "1"

function conduit {
    param(
        [Parameter(Position = 0)]
        [string]$Command,

        [Parameter(Position = 1, ValueFromRemainingArguments = $true)]
        [string[]]$Arguments
    )

    switch ($Command) {
        "" {
            Write-Output "Conduit shell integration active."
        }

        "reload" {
            $config = Join-Path $HOME ".config/conduit/config.toml"

            if (Test-Path $config) {
                Write-Output "Conduit configuration: $config"
            }
        }

        "pwd" {
            (Get-Location).Path
        }

        "workspace" {
            if ($env:CONDUIT_WORKSPACE) {
                $env:CONDUIT_WORKSPACE
            }
            else {
                "default"
            }
        }

        default {
            Write-Error "Unknown Conduit shell command: $Command"
            return 1
        }
    }
}

function conduit-workspace {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name
    )

    $env:CONDUIT_WORKSPACE = $Name
}

function conduit-title {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Title
    )

    [Console]::Write("`e]0;$Title`a")
}

function conduit-notify {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Message
    )

    [Console]::Write("`e]777;notify;$Message`a")
}
