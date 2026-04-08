use windows::core::Result;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};

pub fn init() -> Result<()> {
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok() }
}

pub fn set_master_volume(volume: f32) -> Result<()> {
    unsafe {
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        let endpoint = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)?;
        let current = endpoint.GetMasterVolumeLevelScalar()?;

        if current >= 1.0 {
            return Ok(());
        }

        endpoint.SetMasterVolumeLevelScalar(volume, std::ptr::null())?;
    }

    Ok(())
}
