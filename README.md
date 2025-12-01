# winrt-tts-server

This is a lightweight web server designed for speech synthesis using the Windows Runtime (WinRT) Speech Synthesis API, providing a simple HTTP interface for text-to-speech generation on Windows.

<!-- <img width="1000" alt="image" src="placeholder.png" /> -->

## Features

- 🗣️ **Native Windows TTS**  
  Utilizes the Windows Runtime Speech Synthesis API to generate natural-sounding speech using built-in Windows voices.

- 🔊 **Simple API Design**  
  Offers a minimal HTTP API for generating speech with support for volume, speed, and pitch control.

- 📦 **WAV File Output**  
  Returns synthesized speech as a WAV file in the HTTP response.

- 🎛️ **Voice Parameter Control**  
  Supports adjusting volume, speaking rate, and pitch through the `SpeechSynthesizer.Options` API.

## Use Case

Primarily intended for integration with Discord bots, automation tools, or applications requiring TTS capabilities on Windows without GUI interaction.

## API Details

### `GET /api/voices`

This endpoint returns a list of available voice profiles that can be used with the TTS engine.

#### Response

The response is a JSON array of voice objects. Each object contains:

- `id` *(string)*: A unique identifier for the voice.
- `display_name` *(string)*: The display name of the voice.
- `language` *(string)*: The language code of the voice (e.g., "en-US", "ja-JP").
- `description` *(string)*: The description of the voice.
- `gender` *(string)*: The gender of the voice. ("Male" or "Female").


### `POST /api/tts`

This endpoint generates speech audio from the provided text and voice parameters.

#### Request

The request must be a JSON object with the following fields:

- `text` *(string)*: The input text to be synthesized.
- `voice_id` *(string)*: The identifier of the voice to use.
- `audio_volume` *(number)* *(optional)*: Controls loudness (0.0 - 1.0, default 1.0).
- `speaking_rate` *(number)* *(optional)*: Adjusts speaking rate (0.5 - 6.0, default 1.0).
- `audio_pitch` *(number)* *(optional)*: Modifies pitch (0.0 - 2.0, default 1.0).

#### Response

- `200 OK`: Returns a WAV file containing the synthesized speech.
- `500 INTERNAL_SERVER_ERROR`: Returns a plain-text error message.

## Configuration

You can configure the server using command-line arguments.

| Argument | Default | Description |
| :--- | :--- | :--- |
| `--listen` | `0.0.0.0:3000` | Socket address to bind to. |

## Building & Running

```bash
# Build
cargo build --release

# Run
./target/release/winrt-tts-server --listen 127.0.0.1:3000
```
