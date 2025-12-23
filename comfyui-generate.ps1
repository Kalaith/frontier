# ComfyUI Image Generation Script
# Usage: .\comfyui-generate.ps1 -Prompt "your prompt here" -Width 512 -Height 512 -OutputPath "output.png"
#
# Available Models:
# - juggernautXL_v8Rundiffusion.safetensors (default)
# - plantMilkModelSuite_walnut.safetensors
# - ponyDiffusionV6XL_v6StartWithThisOne.safetensors

param(
    [Parameter(Mandatory=$true)]
    [string]$Prompt,
    
    [Parameter(Mandatory=$false)]
    [int]$Width = 512,
    
    [Parameter(Mandatory=$false)]
    [int]$Height = 512,
    
    [Parameter(Mandatory=$false)]
    [string]$NegativePrompt = "bad hands, blurry, low quality",
    
    [Parameter(Mandatory=$false)]
    [string]$OutputPath = "generated_image.png",
      [Parameter(Mandatory=$false)]
    [string]$ComfyUIServer = "127.0.0.1:8188",
    
    [Parameter(Mandatory=$false)]
    [int]$Steps = 20,
    
    [Parameter(Mandatory=$false)]
    [double]$CFG = 8.0,
      [Parameter(Mandatory=$false)]
    [int]$Seed = -1,
    
    [Parameter(Mandatory=$false)]
    [string]$Model = "juggernautXL_v8Rundiffusion.safetensors",
    
    [Parameter(Mandatory=$false)]
    [string]$Sampler = "euler",
      [Parameter(Mandatory=$false)]
    [string]$Scheduler = "normal",
      [Parameter(Mandatory=$false)]
    [switch]$UseScreenResolution = $false,
    
    [Parameter(Mandatory=$false)]
    [switch]$SetAsWallpaper = $false,
    
    [Parameter(Mandatory=$false)]
    [switch]$OpenImage = $false
)

# Generate random seed if not provided
if ($Seed -eq -1) {
    $Seed = Get-Random -Minimum 1 -Maximum 2147483647
}

# Function to get screen resolution
function Get-ScreenResolution {
    try {
        # Use WMI to get screen resolution
        $monitor = Get-WmiObject -Class Win32_VideoController | Where-Object { $_.CurrentHorizontalResolution -ne $null } | Select-Object -First 1
        if ($monitor) {
            return @{
                Width = $monitor.CurrentHorizontalResolution
                Height = $monitor.CurrentVerticalResolution
            }
        }
        
        # Fallback to .NET method
        Add-Type -AssemblyName System.Windows.Forms
        $screen = [System.Windows.Forms.Screen]::PrimaryScreen
        return @{
            Width = $screen.Bounds.Width
            Height = $screen.Bounds.Height
        }
    }
    catch {
        Write-Warning "Could not detect screen resolution, using defaults"
        return @{ Width = 1920; Height = 1080 }
    }
}

# Function to set wallpaper
function Set-Wallpaper {
    param([string]$ImagePath)
    
    try {
        # Convert to absolute path
        $absolutePath = (Resolve-Path $ImagePath).Path
        
        # Use Windows API to set wallpaper
        Add-Type -TypeDefinition @"
            using System;
            using System.Runtime.InteropServices;
            
            public class Wallpaper {
                [DllImport("user32.dll", CharSet = CharSet.Auto)]
                public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
                
                public static void SetWallpaper(string path) {
                    SystemParametersInfo(20, 0, path, 3);
                }
            }
"@
        
        [Wallpaper]::SetWallpaper($absolutePath)
        Write-Host "Wallpaper set successfully!" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Warning "Failed to set wallpaper: $($_.Exception.Message)"
        return $false
    }
}

# Apply screen resolution if requested
if ($UseScreenResolution) {
    Write-Host "Detecting screen resolution..." -ForegroundColor Blue
    $screenRes = Get-ScreenResolution
    $Width = $screenRes.Width
    $Height = $screenRes.Height
    Write-Host "Using screen resolution: ${Width}x${Height}" -ForegroundColor Green
}

# Generate unique client ID
$ClientId = [System.Guid]::NewGuid().ToString()

Write-Host "ComfyUI Image Generator" -ForegroundColor Cyan
Write-Host "Prompt: $Prompt" -ForegroundColor Yellow
Write-Host "Size: ${Width}x${Height}" -ForegroundColor Green
Write-Host "Seed: $Seed" -ForegroundColor Magenta
Write-Host "Server: $ComfyUIServer" -ForegroundColor Blue
if ($UseScreenResolution) {
    Write-Host "Screen Resolution Mode: Enabled" -ForegroundColor Cyan
}
if ($SetAsWallpaper) {
    Write-Host "Set as Wallpaper: Enabled" -ForegroundColor Cyan
}

# ComfyUI workflow JSON template
$WorkflowTemplate = @{
    "3" = @{
        "class_type" = "KSampler"
        "inputs" = @{
            "cfg" = $CFG
            "denoise" = 1
            "latent_image" = @("5", 0)
            "model" = @("4", 0)
            "negative" = @("7", 0)
            "positive" = @("6", 0)
            "sampler_name" = $Sampler
            "scheduler" = $Scheduler
            "seed" = $Seed
            "steps" = $Steps
        }
    }
    "4" = @{
        "class_type" = "CheckpointLoaderSimple"
        "inputs" = @{
            "ckpt_name" = $Model
        }
    }
    "5" = @{
        "class_type" = "EmptyLatentImage"
        "inputs" = @{
            "batch_size" = 1
            "height" = $Height
            "width" = $Width
        }
    }
    "6" = @{
        "class_type" = "CLIPTextEncode"
        "inputs" = @{
            "clip" = @("4", 1)
            "text" = $Prompt
        }
    }
    "7" = @{
        "class_type" = "CLIPTextEncode"
        "inputs" = @{
            "clip" = @("4", 1)
            "text" = $NegativePrompt
        }
    }
    "8" = @{
        "class_type" = "VAEDecode"
        "inputs" = @{
            "samples" = @("3", 0)
            "vae" = @("4", 2)
        }
    }
    "9" = @{
        "class_type" = "SaveImage"
        "inputs" = @{
            "filename_prefix" = "PowerShell_Generated"
            "images" = @("8", 0)
        }
    }
}

function Queue-Prompt {
    param([hashtable]$Workflow)
    
    try {
        $promptData = @{
            "prompt" = $Workflow
            "client_id" = $ClientId
        } | ConvertTo-Json -Depth 10
        
        $response = Invoke-RestMethod -Uri "http://$ComfyUIServer/prompt" -Method Post -Body $promptData -ContentType "application/json"
        return $response.prompt_id
    }
    catch {
        Write-Error "Failed to queue prompt: $($_.Exception.Message)"
        return $null
    }
}

function Wait-ForCompletion {
    param([string]$PromptId)
    
    Write-Host "Waiting for image generation to complete..." -ForegroundColor Yellow
    
    $maxAttempts = 60  # 5 minutes max
    $attempts = 0
    
    while ($attempts -lt $maxAttempts) {
        try {
            $history = Invoke-RestMethod -Uri "http://$ComfyUIServer/history/$PromptId" -Method Get
            
            if ($history.$PromptId) {
                Write-Host "Generation completed!" -ForegroundColor Green
                return $history.$PromptId
            }
            
            Start-Sleep -Seconds 5
            $attempts++
            Write-Host "." -NoNewline -ForegroundColor Gray
        }
        catch {
            Write-Host "Checking status..." -ForegroundColor Yellow
            Start-Sleep -Seconds 5
            $attempts++
        }
    }
    
    Write-Error "Timeout waiting for completion"
    return $null
}

function Download-GeneratedImage {
    param([object]$HistoryData, [string]$OutputFile)
    
    try {
        # Find the SaveImage node output
        $imageInfo = $null
        foreach ($nodeId in $HistoryData.outputs.PSObject.Properties.Name) {
            $nodeOutput = $HistoryData.outputs.$nodeId
            if ($nodeOutput.images) {
                $imageInfo = $nodeOutput.images[0]
                break
            }
        }
        
        if (-not $imageInfo) {
            Write-Error "No image found in generation output"
            return $false
        }
        
        # Construct download URL
        $filename = $imageInfo.filename
        $subfolder = $imageInfo.subfolder
        $type = $imageInfo.type
        
        $downloadUrl = "http://$ComfyUIServer/view"
        $queryParams = @()
        $queryParams += "filename=$([System.Web.HttpUtility]::UrlEncode($filename))"
        if ($subfolder) { $queryParams += "subfolder=$([System.Web.HttpUtility]::UrlEncode($subfolder))" }
        $queryParams += "type=$([System.Web.HttpUtility]::UrlEncode($type))"
        
        $downloadUrl += "?" + ($queryParams -join "&")
        
        Write-Host "Downloading image: $filename" -ForegroundColor Blue
        
        # Download the image
        Invoke-WebRequest -Uri $downloadUrl -OutFile $OutputFile -UseBasicParsing
        
        if (Test-Path $OutputFile) {
            $fileSize = (Get-Item $OutputFile).Length
            Write-Host "Image saved: $OutputFile ($([math]::Round($fileSize/1KB, 2)) KB)" -ForegroundColor Green
            return $true
        }
        else {
            Write-Error "Failed to save image"
            return $false
        }
    }
    catch {
        Write-Error "Failed to download image: $($_.Exception.Message)"
        return $false
    }
}

function Test-ComfyUIConnection {
    try {
        Write-Host "Testing connection to ComfyUI..." -ForegroundColor Blue
        $response = Invoke-WebRequest -Uri "http://$ComfyUIServer/" -Method Get -TimeoutSec 10 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-Host "ComfyUI is accessible" -ForegroundColor Green
            return $true
        }
    }
    catch {
        Write-Error "Cannot connect to ComfyUI at $ComfyUIServer. Make sure ComfyUI is running."
        Write-Host "Try starting ComfyUI with: python main.py --listen" -ForegroundColor Yellow
        return $false
    }
    return $false
}

# Main execution
try {
    # Add System.Web for URL encoding
    Add-Type -AssemblyName System.Web
    
    # Test connection first
    if (-not (Test-ComfyUIConnection)) {
        exit 1
    }
    
    # Queue the prompt
    Write-Host "Queueing generation request..." -ForegroundColor Blue
    $promptId = Queue-Prompt -Workflow $WorkflowTemplate
    
    if (-not $promptId) {
        Write-Error "Failed to queue prompt"
        exit 1
    }
    
    Write-Host "Prompt ID: $promptId" -ForegroundColor Cyan
    
    # Wait for completion
    $historyData = Wait-ForCompletion -PromptId $promptId
    
    if (-not $historyData) {
        Write-Error "Generation failed or timed out"
        exit 1
    }
    
    # Download the generated image
    $success = Download-GeneratedImage -HistoryData $historyData -OutputFile $OutputPath    
    if ($success) {
        
        Write-Host "Image generation completed successfully!" -ForegroundColor Green
        Write-Host "Output: $OutputPath" -ForegroundColor Cyan
        
        # Set as wallpaper if requested
        if ($SetAsWallpaper) {
            Write-Host "Setting as wallpaper..." -ForegroundColor Blue
            Set-Wallpaper -ImagePath $OutputPath
        }
          # Only open the image if explicitly requested
        if ($env:OS -eq "Windows_NT" -and $OpenImage) {
            Write-Host "Opening image..." -ForegroundColor Blue
            Start-Process $OutputPath
        }
    }
    else {
        Write-Error "Failed to download generated image"
        exit 1
    }
}
catch {
    Write-Error "Script failed: $($_.Exception.Message)"
    exit 1
}

Write-Host "`nDone! Thanks for using ComfyUI PowerShell Generator!" -ForegroundColor Magenta
