import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { GetYoutubeVideosReturn, MauriceApi } from "../api";
import { Box, Slider, Typography } from "@mui/material";

export const SongTime = ({
  speaker,
  song,
  color,
}: {
  speaker: string;
  song: GetYoutubeVideosReturn["videos"][0];
  color: string;
}) => {
  const [time, setTime] = useState(0);
  const [maxTime, setMaxTime] = useState(0);
  const settingTime = useRef(false);

  useEffect(() => {
    const interval = setInterval(async () => {
      const song_time = await MauriceApi.postApiGetSongTime({
        speaker,
      });
      if (settingTime.current) {
        return;
      }
      setTime(song_time.time);
    }, 500);
    return () => clearInterval(interval);
  }, [speaker]);

  useEffect(() => {
    const fetchMaxTime = async () => {
      const song_time = await MauriceApi.postApiGetSongDuration({
        speaker,
      });
      setMaxTime(song_time.duration);
    };
    fetchMaxTime();
  }, [speaker, song]);

  const updateTime = useCallback(
    async (newTime: number) => {
      setTime(newTime);
      settingTime.current = true;
      const timeDelta = newTime - time;
      await MauriceApi.postApiSetSongTime({
        speaker,
        song_time: timeDelta,
      });
      settingTime.current = false;
    },
    [speaker, time]
  );

  //00:00, 03:00, 03:30, etc
  const formattedTime = useMemo(() => {
    const minutes = Math.floor(time / 60);
    const seconds = time - minutes * 60;

    return `${minutes.toFixed(0)}:${seconds < 10 ? "0" : ""}${seconds.toFixed(
      0
    )}`;
  }, [time]);

  const formattedMaxTime = useMemo(() => {
    const minutes = Math.floor(maxTime / 60);
    const seconds = maxTime - minutes * 60;

    return `${minutes.toFixed(0)}:${seconds < 10 ? "0" : ""}${seconds.toFixed(
      0
    )}`;
  }, [maxTime]);

  return (
    <Box
      display="flex"
      width="calc(100% - 10)"
      alignItems="center"
      paddingLeft={5}
      paddingRight={5}
      paddingBottom={1}
      gap={2}
    >
      <Slider
        value={time}
        max={maxTime}
        onChange={(_, value) => updateTime(value as number)}
        style={{
          flex: 1,
        }}
      />
      <Typography
        style={{
          width: 60,
          textAlign: "left",
          color: color,
        }}
      >
        {formattedTime}/{formattedMaxTime}
      </Typography>
    </Box>
  );
};
