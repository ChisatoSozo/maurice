import { useCallback, useEffect, useState } from "react";
import { GetYoutubeVideosReturn, MauriceApi } from "../api";
import { GetYoutubeVideo } from "./GetYoutubeVideo";
import { Playlist } from "./Playlist";
import { Slider } from "@mui/material";

export const Speaker = ({ name }: { name: string }) => {
  const playSong = async (song: GetYoutubeVideosReturn["videos"][0]) => {
    await MauriceApi.postApiPlayAudio({
      song: song,
      speaker: name,
    });
  };

  const setVolumeServer = useCallback(
    async (volume: number) => {
      await MauriceApi.postApiSetVolume({
        volume: volume,
        speaker: name,
      });
    },
    [name]
  );

  const getVolumeServer = useCallback(async () => {
    const volume = await MauriceApi.postApiGetVolume({
      speaker: name,
    });
    return volume;
  }, [name]);

  const [volume, setVolume] = useState<number | null>(null);

  useEffect(() => {
    const fetchVolume = async () => {
      const volume = await getVolumeServer();
      setVolume(volume.volume);
    };

    fetchVolume();
  }, [getVolumeServer]);

  useEffect(() => {
    const fetchSetVolume = async () => {
      if (volume === null) {
        return;
      }
      await setVolumeServer(volume);
    };
    fetchSetVolume();
  }, [volume, setVolumeServer]);

  return (
    <>
      {volume !== null && (
        <Slider
          aria-label="Volume"
          value={volume}
          min={0}
          max={100}
          step={1}
          onChange={(_, value) => {
            setVolume(value as number);
          }}
        />
      )}
      <Playlist />
      <GetYoutubeVideo onPlayVideo={playSong} />
    </>
  );
};
