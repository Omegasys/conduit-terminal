# Conduit PowerShell terminal integration.
#
# This integrates with the PowerShell prompt rather than replacing
# PowerShell's history system.

if ($env:CONDUIT_POWERSHELL_INTEGRATION -eq "1") {
    return
}

$env:CONDUIT_POWERSHELL_INTEGRATION = "1"

function global:__ConduitWriteOsc {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Value
    )

    [Console]::Write("`e]$Value`a")
}

function global:__ConduitPrompt {
    $location = (Get-Location).Path

    __ConduitWriteOsc "7;file://$env:COMPUTERNAME$location"
    __ConduitWriteOsc "133;B"

    if (Get-Command "prompt" -ErrorAction SilentlyContinue) {
        $existingPrompt = Microsoft.PowerShell.Utility\Get-Variable `
            -Name "ConduitOriginalPrompt" `
            -Scope Global `
            -ErrorAction SilentlyContinue

        if ($existingPrompt) {
            & $existingPrompt.Value
        }
        else {
            "PS $location> "
        }
    }
    else {
        "PS $location> "
    }
}

if (Get-Variable -Name prompt -Scope Global -ErrorAction SilentlyContinue) {
    $global:ConduitOriginalPrompt = $global:prompt
}

function global:prompt {
    $status = $global:LASTEXITCODE

    __ConduitWriteOsc "133;D;$status"

    $location = (Get-Location).Path
    __ConduitWriteOsc "7;file://$env:COMPUTERNAME$location"
    __ConduitWriteOsc "133;B"

    if ($global:ConduitOriginalPrompt) {
        & $global:ConduitOriginalPrompt
    }
    else {
        "PS $location> "
    }
}

__ConduitWriteOsc "7;file://$env:COMPUTERNAME$((Get-Location).Path)"
