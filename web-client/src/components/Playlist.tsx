import { GetPlaylistReturn, MauriceApi } from "../api";
import { MusicEntry } from "./MusicEntry";
import { Box, Button } from "@mui/material";
import { Delete, Pause, PlayArrow } from "@mui/icons-material";
import { useEffect } from "react";

export const Playlist = ({
  speaker,
  playlist,
  remove,
}: {
  speaker: string;
  playlist: GetPlaylistReturn["songs"];
  remove: (index: number) => void;
}) => {
  const resume = async () => {
    MauriceApi.postApiResume({
      speaker,
    });
  };

  const pause = async () => {
    MauriceApi.postApiPause({
      speaker,
    });
  };

  useEffect(() => {
    const interval = setInterval(async () => {
      const time = await MauriceApi.postApiGetSongTime({
        speaker,
      });
      console.log(time.song_time);
    }, 500);
    return () => clearInterval(interval);
  }, [speaker]);

  return playlist.map((song, i) => (
    <MusicEntry
      speaker={speaker}
      key={song.url}
      video={song}
      showTime={i === 0}
      controls={(color) => (
        <>
          {i === 0 && (
            <>
              <Box>
                <Button
                  style={{
                    height: 60,
                    width: 60,
                    color,
                  }}
                  onClick={pause}
                >
                  <Pause
                    style={{
                      fontSize: 40,
                      marginTop: 23,
                      marginLeft: 24,
                    }}
                  />
                </Button>
              </Box>
              <Box>
                <Button
                  style={{
                    height: 60,
                    width: 60,
                    color,
                  }}
                  onClick={resume}
                >
                  <PlayArrow
                    style={{
                      fontSize: 40,
                      marginTop: 23,
                      marginLeft: 24,
                    }}
                  />
                </Button>
              </Box>
            </>
          )}
          {i !== 0 && (
            <Box>
              <Button
                style={{
                  height: 60,
                  width: 60,
                  color,
                }}
                onClick={() => remove(i)}
              >
                <Delete
                  style={{
                    fontSize: 40,
                    marginTop: 23,
                    marginLeft: 24,
                  }}
                />
              </Button>
            </Box>
          )}
        </>
      )}
    />
  ));
};
