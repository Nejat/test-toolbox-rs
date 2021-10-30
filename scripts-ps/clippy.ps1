function Confirm-Success {
    param([string] $step)

    if ($lastExitCode -ne 0) {
        Write-Host
        Write-Host "$step failed with exit code $lastExitCode" -ForegroundColor Red
        Write-Host
        exit $lastExitCode
    }
}

[string] $script = $PSCommandPath.Split([IO.Path]::DirectorySeparatorChar)[-1];

if (Test-Path -Path $script -PathType Leaf) {
    Write-Host
    Write-Host "Scripts need to be run from the project root, i.e. '.\scripts-ps\$script'" -ForegroundColor Yellow
    Write-Host

    exit -1
}

clear
Write-Host "running clippy actual unoptimized" -ForegroundColor Yellow
cargo clippy --features="actual"
Confirm-Success "clippy actual unoptimized"

Write-Host "running clippy actual optimized" -ForegroundColor Yellow
cargo clippy --release --features="actual"
Confirm-Success "clippy actual optimized"

Write-Host "running clippy capture unoptimized" -ForegroundColor Yellow
cargo clippy --features="capture"
Confirm-Success "clippy capture unoptimized"

Write-Host "running clippy capture optimized" -ForegroundColor Yellow
cargo clippy --release --features="capture"
Confirm-Success "clippy capture optimized"

Write-Host "running clippy expected unoptimized" -ForegroundColor Yellow
cargo clippy --features="expected"
Confirm-Success "clippy expected unoptimized"

Write-Host "running clippy expected optimized" -ForegroundColor Yellow
cargo clippy --release --features="expected"
Confirm-Success "clippy expected optimized"
