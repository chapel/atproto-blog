$watchexecPath = (Get-Command watchexec).Path
$scriptPath = Join-Path $PSScriptRoot "watch-wrapper.ps1"

Start-Process powershell `
  -ArgumentList "-NoProfile", "-NonInteractive", "-File", $scriptPath, $watchexecPath `
  -NoNewWindow

Start-Sleep -Seconds 1

$processId = (Get-Process watchexec -ErrorAction SilentlyContinue).Id
if ($processId) {
  $processId | Out-File -FilePath ".watch-pid"
}

Start-Sleep -Seconds 1