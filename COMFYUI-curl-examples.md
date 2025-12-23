# ComfyUI — curl examples to generate an image ✅

Short and ready-to-copy curl commands you can paste into a terminal to queue an image generation, poll for completion, and download the resulting image.

---

## Requirements
- ComfyUI running and listening (default: `http://127.0.0.1:8188`)  
- Optional: `jq` (for JSON parsing) — helpful for the polling/download examples

---

## 1) Queue a generation (single prompt)
This posts a ComfyUI workflow JSON to the `/prompt` endpoint and returns a `prompt_id`.

Bash (Linux / WSL / Git Bash / macOS):

```bash
CLIENT_ID=$(uuidgen)  # or any unique string
curl -s -X POST "http://127.0.0.1:8188/prompt" \
  -H "Content-Type: application/json" \
  -d @- <<'JSON'
{
  "prompt": {
    "3": {
      "class_type": "KSampler",
      "inputs": {
        "cfg": 8.0,
        "denoise": 1,
        "latent_image": ["5", 0],
        "model": ["4", 0],
        "negative": ["7", 0],
        "positive": ["6", 0],
        "sampler_name": "euler",
        "scheduler": "normal",
        "seed": 123456,
        "steps": 20
      }
    },
    "4": { "class_type": "CheckpointLoaderSimple", "inputs": { "ckpt_name": "juggernautXL_v8Rundiffusion.safetensors" } },
    "5": { "class_type": "EmptyLatentImage", "inputs": { "batch_size": 1, "height": 512, "width": 512 } },
    "6": { "class_type": "CLIPTextEncode", "inputs": { "clip": ["4", 1], "text": "A dramatic sunset over a fantasy castle" } },
    "7": { "class_type": "CLIPTextEncode", "inputs": { "clip": ["4", 1], "text": "bad hands, blurry, low quality" } },
    "8": { "class_type": "VAEDecode", "inputs": { "samples": ["3", 0], "vae": ["4", 2] } },
    "9": { "class_type": "SaveImage", "inputs": { "filename_prefix": "curl_generated", "images": ["8", 0] } }
  },
  "client_id": "'"$CLIENT_ID"'"
}
JSON
```

The response will contain a JSON object like:

```json
{ "prompt_id": "abc123..." }
```

---

## 2) Poll for completion (bash + jq)
Replace `<PROMPT_ID>` with the returned id:

```bash
PROMPT_ID="abc123..."   # set from the previous response
while true; do
  history=$(curl -s "http://127.0.0.1:8188/history/$PROMPT_ID")
  # the API returns an object keyed by prompt_id when complete
  if echo "$history" | jq -e ".\"$PROMPT_ID\"" >/dev/null 2>&1; then
    echo "Generation finished"
    break
  fi
  echo "Waiting..."
  sleep 5
done
```

---

## 3) Extract image info and download
After completion, extract the first image info (filename/subfolder/type) and download it:

```bash
# extract image metadata from history
FILE=$(echo "$history" | jq -r ".\"$PROMPT_ID\".outputs | .[] | select(.images != null) | .images[0].filename")
SUBFOLDER=$(echo "$history" | jq -r ".\"$PROMPT_ID\".outputs | .[] | select(.images != null) | .images[0].subfolder")
TYPE=$(echo "$history" | jq -r ".\"$PROMPT_ID\".outputs | .[] | select(.images != null) | .images[0].type")

# download (URL-encode params automatically with curl --data-urlencode when using -G)
curl -s -G "http://127.0.0.1:8188/view" \
  --data-urlencode "filename=$FILE" \
  --data-urlencode "subfolder=$SUBFOLDER" \
  --data-urlencode "type=$TYPE" \
  -o "generated_image.${TYPE}"
```

---

## PowerShell (Windows) equivalents
- Create GUID: `$clientId = [guid]::NewGuid().ToString()`  
- POST: use `Invoke-RestMethod -Uri "http://127.0.0.1:8188/prompt" -Method Post -Body ($payload | ConvertTo-Json -Depth 10) -ContentType "application/json"`  
- Poll with `Invoke-RestMethod` and inspect the response for the prompt ID key; download with `Invoke-WebRequest`.

---

## Notes & tips 💡
- Change **height**, **width**, **model**, **steps**, **seed** and **prompt text** in the payload to tune results.  
- If you don't set a seed, scripts typically choose a random seed; for reproducible runs set a fixed seed.  
- If `jq` or `uuidgen` aren't available on Windows, use PowerShell's `ConvertFrom-Json` / `ConvertTo-Json` and `[guid]::NewGuid()` instead.

---

If you'd like, I can also add a one-file example script (Bash or PowerShell) that wraps these steps and saves the image automatically—tell me which you prefer. 🔧
