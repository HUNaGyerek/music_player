
$regFilePath = "$(Join-Path $PSScriptRoot 'register_file_associations.reg')"

Write-Output "Registering file associations using: $regFilePath"

# Ensure the file exists
if (Test-Path $regFilePath) {
    Start-Process regedit.exe -ArgumentList "/s $regFilePath" -Verb RunAs
    Write-Output "Registry file processed successfully."
} else {
    Write-Output "Registry file not found: $regFilePath"
}