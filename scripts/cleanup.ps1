if (Test-Path ".watch-pid") { 
  Remove-Item ".watch-pid"
}
if (Test-Path "watchexec.log") { 
  Remove-Item "watchexec.log"
}
Get-Process watchexec -ErrorAction SilentlyContinue | 
Stop-Process -Force -ErrorAction SilentlyContinue
exit 0 