
$baseUrl = "http://127.0.0.1:8188"
$outputDir = "$PSScriptRoot\assets\images"

# Ensure output directory exists
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir | Out-Null
}

function Get-FirstCheckpoint {
    try {
        $info = Invoke-RestMethod -Uri "$baseUrl/object_info/CheckpointLoaderSimple" -Method Get
        $ckpt = $info.CheckpointLoaderSimple.input.required.ckpt_name[0]
        if ($ckpt -is [array]) {
            return $ckpt[0]
        }
        return $ckpt
    } catch {
        Write-Error "Failed to get checkpoints. Is ComfyUI running at $baseUrl?"
        exit 1
    }
}

function Generate-Image {
    param(
        [string]$PromptText,
        [string]$Filename,
        [string]$Checkpoint
    )

    $clientId = [guid]::NewGuid().ToString()
    
    # Construct Workflow JSON
    $prompt = @{
        "3" = @{
            "class_type" = "KSampler"
            "inputs" = @{
                "cfg" = 8.0
                "denoise" = 1
                "latent_image" = @("5", 0)
                "model" = @("4", 0)
                "negative" = @("7", 0)
                "positive" = @("6", 0)
                "sampler_name" = "euler"
                "scheduler" = "normal"
                "seed" = (Get-Random)
                "steps" = 20
            }
        }
        "4" = @{
            "class_type" = "CheckpointLoaderSimple"
            "inputs" = @{
                "ckpt_name" = $Checkpoint
            }
        }
        "5" = @{
            "class_type" = "EmptyLatentImage"
            "inputs" = @{
                "batch_size" = 1
                "height" = 512
                "width" = 512
            }
        }
        "6" = @{
            "class_type" = "CLIPTextEncode"
            "inputs" = @{
                "clip" = @("4", 1)
                "text" = $PromptText
            }
        }
        "7" = @{
            "class_type" = "CLIPTextEncode"
            "inputs" = @{
                "clip" = @("4", 1)
                "text" = "bad hands, blurry, low quality, cropped, worst quality"
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
                "filename_prefix" = "frontier_gen"
                "images" = @("8", 0)
            }
        }
    }

    $payload = @{
        "prompt" = $prompt
        "client_id" = $clientId
    }

    try {
        # Queue Prompt
        Write-Host "Queueing generation for $Filename..."
        $response = Invoke-RestMethod -Uri "$baseUrl/prompt" -Method Post -Body ($payload | ConvertTo-Json -Depth 10) -ContentType "application/json"
        $promptId = $response.prompt_id
        
        Write-Host "Prompt ID: $promptId"
        
        # Poll for completion
        while ($true) {
            Start-Sleep -Seconds 1
            $history = Invoke-RestMethod -Uri "$baseUrl/history/$promptId" -Method Get
            if ($history.$promptId) {
                break
            }
            Write-Host -NoNewline "."
        }
        Write-Host "`nGeneration Complete!"

        # Extract info
        $outputs = $history.$promptId.outputs
        # Find the SaveImage output (node 9)
        $images = $outputs."9".images
        $serverFilename = $images[0].filename
        $subfolder = $images[0].subfolder
        $type = $images[0].type
        
        # Download
        Write-Host "Downloading..."
        $url = "$baseUrl/view?filename=$serverFilename&subfolder=$subfolder&type=$type"
        $destPath = Join-Path $outputDir "$Filename.png"
        
        Invoke-WebRequest -Uri $url -OutFile $destPath
        Write-Host "Saved to $destPath"
        
    } catch {
        Write-Error "Error during generation: $_"
    }
}

$checkpoint = Get-FirstCheckpoint
Write-Host "Using Checkpoint: $checkpoint"

Generate-Image -PromptText "A grim, battle-worn fantasy soldier with heavy plate armor and a notched sword, dark fantasy style, digital painting, 2D sprite feel, isolated on a simple dark background. Portrait framing." -Filename "soldier" -Checkpoint $checkpoint
Generate-Image -PromptText "A mystical fantasy healer with hooded robes, glowing runes on hands, carrying a gnarled staff, dark fantasy style, digital painting, 2D sprite feel, isolated on a simple dark background. Portrait framing." -Filename "healer" -Checkpoint $checkpoint
Generate-Image -PromptText "A terrifying forest beast with multiple eyes and moss-covered fur, sharp claws, glowing green eyes, dark fantasy style, digital painting, 2D sprite feel, isolated on a simple dark background. Portrait framing." -Filename "forest_beast" -Checkpoint $checkpoint

# Ensure cards directory exists
$cardsDir = Join-Path $outputDir "cards"
if (-not (Test-Path $cardsDir)) {
    New-Item -ItemType Directory -Path $cardsDir | Out-Null
}

Generate-Image -PromptText "Fantasy card art, heavy shield bash impact, dynamic action, stunned enemy, motion blur, dark fantasy style, digital painting, 2D sprite feel" -Filename "cards\shield_bash" -Checkpoint $checkpoint
Generate-Image -PromptText "Fantasy warrior shouting battle cry to intimidate, closeup face, saliva, fierce expression, terrified enemies in background, dark fantasy style, digital painting, 2D sprite feel" -Filename "cards\intimidate" -Checkpoint $checkpoint

Write-Host "All done!"
