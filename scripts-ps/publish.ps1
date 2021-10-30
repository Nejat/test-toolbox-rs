param(
    [switch] $ForReals = $false,
    [switch] $SkipClean = $false
)

[string] $script = $PSCommandPath.Split([IO.Path]::DirectorySeparatorChar)[-1];

if (Test-Path -Path $script -PathType Leaf) {
    Write-Host
    Write-Host "Scripts need to be run from the project root, i.e. '.\scripts-ps\$script'" -ForegroundColor Yellow
    Write-Host

    exit -1
}

function Confirm-Success {
    param([string] $step)

    if ($lastExitCode -ne 0) {
        Write-Host
        Write-Host "$step failed with exit code $lastExitCode" -ForegroundColor Red
        Write-Host
        exit $lastExitCode
    }
}

clear

if ($ForReals -and $SkipClean) {
    Write-Host Can not skip clean step when actually publishing -ForegroundColor Yellow
    Write-Host

    $SkipClean = $false
}

if (!$SkipClean) {
    cargo clean
    Confirm-Success "clean"
}

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

Write-Host "running test all features unoptimized" -ForegroundColor Yellow
cargo test --features="all" -- --nocapture --test-threads=1
Confirm-Success "test all features unoptimized"

Write-Host "running test all features optimized" -ForegroundColor Yellow
cargo test --release --features="all" -- --nocapture --test-threads=1
Confirm-Success "test all features optimized"

if ($ForReals) {
    Write-Host "running publish" -ForegroundColor Yellow
    cargo publish --locked --all-features
    Confirm-Success "publish"
} else {
    Write-Host "running publish dry run" -ForegroundColor Yellow
    cargo publish --locked --all-features --dry-run
    Confirm-Success "publish dry run"
}
