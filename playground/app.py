import sounddevice as sd
import numpy as np
import soundfile as sf

def list_microphones():
    devices = sd.query_devices()
    input_devices = [device for device in devices if device['max_input_channels'] > 0]
    
    if input_devices:
        print("Available Microphones (Input Devices):")
        for idx, device in enumerate(input_devices):
            print(f"{idx + 1}: {device['name']}")
        return input_devices
    else:
        print("No microphones found.")
        return []

def select_device(devices, target_name="MC1000"):
    for device in devices:
        if target_name in device['name']:
            print(f"Selected microphone: {device['name']}")
            return device['index']  # Return the actual device index
    print(f"Target device {target_name} not found.")
    return None

def record_audio(device_index, duration=10, filename="audio.wav", samplerate=48000):
    try:
        device_info = sd.query_devices(device_index)
        max_input_channels = device_info['max_input_channels']
        print(f"Recording audio for {duration} seconds with {max_input_channels} channels...")
        
        # Ensure that the number of channels does not exceed the device's maximum
        audio_data = sd.rec(int(duration * samplerate), samplerate=samplerate, channels=min(2, max_input_channels), dtype='float64')
        sd.wait()  # Wait for recording to finish

        # Save the recorded audio to a file
        sf.write(filename, audio_data, samplerate)
        print(f"Audio saved to {filename}")
        
    except Exception as e:
        print(f"An error occurred during recording: {e}")


if __name__ == "__main__":
    # List available microphones
    devices = list_microphones()

    print(devices)
    
    # Auto-select the "MC1000" microphone
    mc1000_index = select_device(devices, target_name="MC1000")
    
    if mc1000_index is not None:
        # Record 10 seconds of audio and save to audio.wav
        record_audio(device_index=mc1000_index)
