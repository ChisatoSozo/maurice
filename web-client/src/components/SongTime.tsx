import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { MauriceApi } from "../api";
import { Box, Slider, Typography } from "@mui/material";

export const SongTime = ({ speaker }: { speaker: string }) => {
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
      setTime(
        song_time.song_time < song_time.song_duration
          ? song_time.song_time
          : song_time.song_duration
      );
      setMaxTime(
        song_time.song_duration > song_time.song_time
          ? song_time.song_duration
          : song_time.song_time
      );
    }, 500);
    return () => clearInterval(interval);
  }, [speaker]);

  const updateTime = useCallback(
    async (time: number) => {
      setTime(time);
      settingTime.current = true;
      await MauriceApi.postApiSetSongTime({
        speaker,
        song_time: time,
      });
      settingTime.current = false;
    },
    [setTime, speaker]
  );

  //00:00, 03:00, 03:30, etc
  const formattedTime = useMemo(() => {
    const minutes = Math.floor(time / 60);
    const seconds = time % 60;

    return `${minutes.toFixed(0)}:${seconds < 10 ? "0" : ""}${seconds.toFixed(
      0
    )}`;
  }, [time]);

  const formattedMaxTime = useMemo(() => {
    const minutes = Math.floor(maxTime / 60);
    const seconds = maxTime % 60;

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
        }}
      >
        {formattedTime}/{formattedMaxTime}
      </Typography>
    </Box>
  );
};
