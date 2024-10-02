import { GetPlaylistReturn, MauriceApi } from "../api";
import { MusicEntry } from "./MusicEntry";
import { Box, Button } from "@mui/material";
import { Delete, Pause, PlayArrow, Stop } from "@mui/icons-material";

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

  const stop = async () => {
    MauriceApi.postApiStop({
      speaker,
    });
  };

  // const next = async () => {
  //   MauriceApi.postApiStop({
  //     speaker,
  //   });
  // };

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
              {playlist.length == 1 && (
                <Box>
                  <Button
                    style={{
                      height: 60,
                      width: 60,
                      color,
                    }}
                    onClick={stop}
                  >
                    <Stop />
                  </Button>
                </Box>
              )}
              <Box>
                <Button
                  style={{
                    height: 60,
                    width: 60,
                    color,
                  }}
                  onClick={pause}
                >
                  <Pause />
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
                  <PlayArrow />
                </Button>
              </Box>
              {/* {playlist.length > 1 && (
                <Box>
                  <Button
                    style={{
                      height: 60,
                      width: 60,
                      color,
                    }}
                    onClick={next}
                  >
                    <SkipNext />
                  </Button>
                </Box>
              )} */}
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
                <Delete />
              </Button>
            </Box>
          )}
        </>
      )}
    />
  ));
};
