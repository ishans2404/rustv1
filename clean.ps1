Get-ChildItem -File | Where-Object { $_.Extension -in ".exe", ".pdb" } | ForEach-Object {
    Write-Host "Deleting: $($_.Name)"
    Remove-Item $_.FullName -Force
}
