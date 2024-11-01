param($watchexecPath)
$ErrorActionPreference = 'SilentlyContinue'
$WarningPreference = 'SilentlyContinue'

Set-Location (Split-Path $PSScriptRoot)
Write-Host "Current directory: $(Get-Location)"
Write-Host "Running watchexec..."

$pinfo = New-Object System.Diagnostics.ProcessStartInfo
$pinfo.FileName = $watchexecPath
$pinfo.Arguments = "-n -e rs just build"
$pinfo.RedirectStandardError = $true
$pinfo.RedirectStandardOutput = $true
$pinfo.UseShellExecute = $false
$pinfo.WorkingDirectory = Get-Location

$process = New-Object System.Diagnostics.Process
$process.StartInfo = $pinfo
$process.Start() | Out-Null

$process.Id | Out-File -FilePath ".watch-pid"

# Read output continuously
while (!$process.HasExited) {
    if (!$process.StandardOutput.EndOfStream) {
        $line = $process.StandardOutput.ReadLine()
        $line | Tee-Object -Append -FilePath "watchexec.log"
    }
    if (!$process.StandardError.EndOfStream) {
        $line = $process.StandardError.ReadLine()
        $line | Tee-Object -Append -FilePath "watchexec.log"
    }
}

Write-Host "Command completed with exit code: $($process.ExitCode)"