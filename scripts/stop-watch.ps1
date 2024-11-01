if (Test-Path ".watch-pid") {
  $processId = Get-Content ".watch-pid"
  Get-Process -Id $processId -ErrorAction SilentlyContinue | 
  Stop-Process -Force -ErrorAction SilentlyContinue
  Start-Sleep -Seconds 1
}

if (Test-Path ".watch-pid") { 
  Remove-Item ".watch-pid"
}
if (Test-Path "watchexec.log") { 
  Remove-Item "watchexec.log"
}

exit 0 