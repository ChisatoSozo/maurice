import { useCallback, useEffect, useState } from "react";
import { GetYoutubeVideosReturn, MauriceApi } from "../api";
import { GetYoutubeVideo } from "./GetYoutubeVideo";
import { Playlist } from "./Playlist";
import { Slider } from "@mui/material";

export const Speaker = ({ name }: { name: string }) => {
  const [playlist, setPlaylist] = useState<GetYoutubeVideosReturn["videos"]>(
    []
  );
  const getPlaylist = useCallback(async () => {
    const songs = (
      await MauriceApi.postApiGetPlaylist({
        speaker: name,
      })
    ).songs;

    setPlaylist(songs);
  }, [name]);

  const playSong = useCallback(
    async (song: GetYoutubeVideosReturn["videos"][0]) => {
      await MauriceApi.postApiPlayAudio({
        song: song,
        speaker: name,
      });
      getPlaylist();
    },
    [getPlaylist, name]
  );

  const addSong = useCallback(
    async (song: GetYoutubeVideosReturn["videos"][0]) => {
      await MauriceApi.postApiAppendSongToPlaylist({
        song: song,
        speaker: name,
      });
      getPlaylist();
    },
    [getPlaylist, name]
  );

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

  const remove = useCallback(
    async (index: number) => {
      await MauriceApi.postApiRemoveSongFromPlaylistAtIndex({
        speaker: name,
        index,
      });
      getPlaylist();
    },
    [getPlaylist, name]
  );

  const [volume, setVolume] = useState<number | null>(null);

  useEffect(() => {
    const fetchSetVolume = async () => {
      if (volume === null) {
        return;
      }
      await setVolumeServer(volume);
    };
    fetchSetVolume();
  }, [volume, setVolumeServer]);

  useEffect(() => {
    //occasionally fetch volume and playlist
    const fetchStuff = async () => {
      const fetchVolume = async () => {
        const volume = await getVolumeServer();
        setVolume(volume.volume);
      };
      getPlaylist();
      fetchVolume();
    };
    fetchStuff();
    const interval = setInterval(() => {
      fetchStuff();
    }, 2000);
    return () => clearInterval(interval);
  }, [getVolumeServer, getPlaylist]);

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
      <Playlist speaker={name} playlist={playlist} remove={remove} />
      <GetYoutubeVideo
        onPlayVideo={playSong}
        onAddVideo={addSong}
        speaker={name}
      />
    </>
  );
};
